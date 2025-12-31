# Investigation: scroll_line_down 102ms performance

## Root Bead
ID: cclv-4pp
Status: in_progress
Branch: 002-view-state-layer

## Symptom
scroll_line_down benchmark takes ~102ms, needs to be ~1ms (100x faster).
Reproduction: `cargo bench --profile release --features="bench-internals" --bench scroll_benchmark`

## Root Cause IDENTIFIED

**Primary bottleneck: AppState deep clone on every scroll (30%+ overhead)**

Location: `src/view/mod.rs:474`
```rust
scroll_handler::handle_scroll_action(self.app_state.clone(), action, viewport);
```

The comment "This is safe because AppState is cheap to clone (Rc internals)" is **FALSE**.
AppState uses `#[derive(Clone)]` which does DEEP COPIES of:
- Vec<SessionViewState>
- Vec<ConversationViewState>
- Vec<EntryView> (31k entries with pre-rendered `Vec<Line>`)

## Flamegraph Analysis

### Profiling Methodology Caveat

**WARNING: `--profile-time` is misleading for iter_batched benchmarks!**

We tried two profiling approaches:

1. **Full benchmark** (`cargo flamegraph --bench scroll_benchmark`):
   - Captures entire process including cargo build, fixture load, all iterations
   - iter_batched separates setup from measurement, but perf profiles everything
   - Shows ~155k samples, mix of setup and measurement

2. **Profile-time mode** (`-- --profile-time 10`):
   - Runs benchmark in tight loop for specified seconds
   - **DOES NOT separate setup from measurement** - runs both together
   - Shows ~1.3k samples with misleading `recompute_lines` at 11%

The `--profile-time` flamegraph showed `EntryView::recompute_lines` at 11%, which initially
suggested syntax highlighting was being re-run during scroll. **This was wrong.**

Investigation revealed:
- Debug logging showed 13,054 calls to `set_viewport()` during benchmark
- This is 122 sessions × ~107 iterations of setup
- Each `new_for_bench()` call triggers `set_viewport_all()` → `relayout()` on all sessions
- The `recompute_lines` overhead is **setup cost**, not scroll cost

**Syntax highlighting is cached in `EntryView.rendered_lines` and NOT re-computed on scroll.**

### What's in the MEASURED path (the actual bottleneck):

| Category | % | Notes |
|----------|---|-------|
| **State cloning** | **30%** | `app_state.clone()` at mod.rs:474 - IN HOT PATH |
| Memory mgmt | 10% | Alloc/dealloc from clone operations |
| Text cloning | 5% | Span/Cow clones from Vec<Line> copies |

### What's in SETUP only (not the bottleneck):

| Category | % | Notes |
|----------|---|-------|
| Syntax highlighting | 13% | One-time cost in `compute_entry_lines()` |
| Relayout | 11% | `set_viewport_all()` in `new_for_bench()` |
| | | Loops over 122 sessions × subagents per iteration |

## Hypotheses

### H4: AppState deep clone on every scroll [LEADING - CONFIRMED]
- Bead: cclv-4pp.6
- Evidence: E1 (flamegraph shows 30%+ in clone operations in measured path)
- Fix: Change handle_scroll_action to take `&mut AppState` instead of owned

### H1: Vec allocation in visible_range() [ELIMINATED]
- Bead: cclv-4pp.1 (closed)
- Eliminated by: E1 - flamegraph shows no significant Vec::collect overhead

### H2: HeightIndex::prefix_sum() is expensive [ELIMINATED]
- Bead: cclv-4pp.2 (closed)
- Eliminated by: E1 - no Fenwick tree operations visible in profile

### H3: Rendering overhead is the actual bottleneck [ELIMINATED]
- Bead: cclv-4pp.3
- Eliminated: Rendering is only ~3%, syntax highlighting is cached (not re-run on scroll)
- The 13% syntect in flamegraph is setup cost, not scroll cost

## Evidence Log

| ID | Source | Finding | Supports | Refutes |
|----|--------|---------|----------|---------|
| E1 | cclv-4pp.5 | Flamegraph: ConversationViewState::clone 15%, to_vec_in 15%, drop 10% | H4 | H1, H2, H3 |
| E2 | Debug log | 13,054 set_viewport calls = setup overhead, not scroll | H4 | - |

## Dead Ends
- H1: Vec allocation - NOT the bottleneck (E1)
- H2: Fenwick tree - NOT visible in profile (E1)
- H3: Rendering/highlighting - NOT in hot path, cached in EntryView (E1, E2)
- `--profile-time` flamegraph - misleading, includes setup (E2)

## Recommended Fix

**Single fix needed: Eliminate state cloning**

1. Change `handle_scroll_action(state: AppState)` to `handle_scroll_action(state: &mut AppState)`
2. Update call site at mod.rs:474 to pass `&mut self.app_state`
3. Remove the false comment about "Rc internals"

Expected improvement: ~30% reduction (102ms → ~70ms)

Further optimization to reach <2ms target would require:
- Investigating the remaining 70ms
- Possible architectural changes (Rc for expensive data, incremental updates)

## Investigation Artifacts
- `cclv-4pp-flamegraph.svg` - Full benchmark flamegraph with debug symbols
- Cargo.toml `[profile.bench]` added for future profiling
- `/tmp/cclv_debug.log` - Debug output showing set_viewport call counts

## Profiling Recommendations

For future profiling of iter_batched benchmarks:
1. **Don't use `--profile-time`** - it conflates setup with measurement
2. Use full `cargo flamegraph` but mentally subtract setup costs
3. Add targeted debug logging to verify what's actually in the hot path
4. Consider creating a minimal reproduction that isolates just the measured code

## Next Steps
1. Implement the &mut AppState fix
2. Re-run benchmark to measure improvement
3. Profile again if still >2ms to find next bottleneck
