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
- Vec<EntryView> (31k entries with rendered content)

**Flamegraph Breakdown (total 102ms):**

| Category | % | Time |
|----------|---|------|
| State cloning | 30% | ~31ms |
| Syntax highlighting | 13% | ~13ms |
| Memory mgmt (alloc/dealloc) | 10% | ~10ms |
| Text cloning (Span/Cow) | 5% | ~5ms |
| Rendering | 3% | ~3ms |
| Other | 39% | ~40ms |

## Hypotheses

### H4: AppState deep clone on every scroll [LEADING - CONFIRMED]
- Bead: cclv-4pp.6
- Evidence: E1 (flamegraph shows 30%+ in clone operations)
- Fix: Change handle_scroll_action to take `&mut AppState` instead of owned

### H1: Vec allocation in visible_range() [ELIMINATED]
- Bead: cclv-4pp.1 (closed)
- Eliminated by: E1 - flamegraph shows no significant Vec::collect overhead

### H2: HeightIndex::prefix_sum() is expensive [ELIMINATED]
- Bead: cclv-4pp.2 (closed)
- Eliminated by: E1 - no Fenwick tree operations visible in profile

### H3: Rendering overhead is the actual bottleneck [PARKED]
- Bead: cclv-4pp.3
- Partial: Rendering is only ~3%, but syntax highlighting is ~13%
- Secondary optimization target

## Evidence Log

| ID | Source | Finding | Supports | Refutes |
|----|--------|---------|----------|---------|
| E1 | cclv-4pp.5 | Flamegraph: ConversationViewState::clone 15%, to_vec_in 15%, drop 10% | H4 | H1, H2 |

## Dead Ends
- H1: Vec allocation - NOT the bottleneck (E1)
- H2: Fenwick tree - NOT visible in profile (E1)

## Recommended Fix

**Phase 1: Eliminate state cloning (target: <35ms)**
1. Change `handle_scroll_action(state: AppState)` to `handle_scroll_action(state: &mut AppState)`
2. Update call sites to pass mutable reference
3. Remove the false comment about Rc internals

**Phase 2: Optimize syntax highlighting (target: <22ms)**
- Cache syntax highlighting results instead of re-highlighting on every render
- Consider lazy highlighting (only visible lines)

**Phase 3: Further optimization (target: <2ms)**
- May require architectural changes (Rc for expensive data, incremental updates)

## Investigation Artifacts
- `flamegraph.svg` - Generated with debug symbols
- `perf.data` - Raw perf data (5.7GB)

## Next Steps
1. Close cclv-4pp.4 experiment as complete
2. Create implementation task for Phase 1 fix
3. Re-run benchmark after fix to measure improvement
