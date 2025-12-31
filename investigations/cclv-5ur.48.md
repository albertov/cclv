# Investigation: Subagent panes not behaving like main agent pane

## Root Bead
ID: cclv-5ur.48
Status: in_progress → INVESTIGATION COMPLETE
Branch: 002-view-state-layer

## Symptom
Two manifestations of underlying architecture issue:
1. **Vertical text rendering**: When expanding entries in subagent panes, text renders one char per line
2. **Mouse events broken**: Clicking entries in subagent panes doesn't toggle expand/collapse

Reproduction:
1. `cargo run --release -- tests/fixtures/cc-session-log.jsonl`
2. Press `]` to switch to subagent tab
3. Press Enter to expand → vertical text (Bug 1)
4. Click entry → nothing happens (Bug 2)

Test: `tests/view_snapshots.rs::bug_subagent_entry_expand_collapse_not_working`

## User's Insight
"smell like we have an architecture problem" - **CONFIRMED**. This is not two separate bugs but ONE architectural issue manifesting in multiple ways.

## ROOT CAUSE ANALYSIS

### Bug 1: Vertical Text on Expand (H3 CONFIRMED)

**Root Cause**: `subagent_mut()` in `session.rs:83-96` creates `ConversationViewState` with `viewport_width=0`, but no relayout is triggered afterward. When `toggle_entry_expanded()` uses this width to recompute lines, each character becomes its own line.

**Code Path**:
1. `new_for_test()` relayouts existing subagents (count=0 at construction)
2. `add_entries()` creates new subagent via `subagent_mut()`
3. `subagent_mut()` creates `ConversationViewState::new()` with `viewport_width=0`
4. `toggle_entry_expanded()` calls `recompute_lines(self.viewport_width)` → width=0
5. Text wraps per character → vertical rendering

**Key Evidence**:
- `conversation.rs:108`: `viewport_width: 0` in constructor
- `conversation.rs:636`: `entry.recompute_lines(effective_wrap, self.viewport_width, is_focused)`
- `session.rs:86-92`: `subagent_mut()` creates without relayout

### Bug 2: Mouse Clicks Don't Work (H1 CONFIRMED)

**Root Cause**: `detect_entry_click()` in `mouse_handler.rs:125-177` has dead code. After unified tab layout (FR-083), `calculate_pane_areas()` always returns `None` for `subagent_pane_area`. The subagent detection block never executes; all clicks go through main pane logic.

**Code Path**:
1. `calculate_pane_areas()` returns `(conversation_area, None)`
2. `detect_entry_click()` checks `if let Some(subagent_area) = subagent_pane_area`
3. Condition is always false → falls through to main pane detection
4. Main pane detection queries main conversation's `hit_test()`
5. Wrong conversation's entries are tested → clicks don't register

**Key Evidence**:
- `layout.rs:124-125`: `// FR-083: No horizontal split` → returns `(area, None)`
- `mouse_handler.rs:125-177`: Dead code block (never executes)

### Architectural Issue

**SessionViewState doesn't track viewport_width**, so it can't propagate relayout to dynamically created subagents. The viewport width is only known at the TuiApp layer.

**Pattern**: This is the 4th instance of "routing to wrong conversation" bugs:
1. cclv-5ur.40.2: Header agent label
2. cclv-5ur.42: Scroll routing
3. cclv-5ur.47: Expand routing (FIXED - uses selected_tab correctly)
4. cclv-5ur.48: Mouse routing (ACTIVE)

## Hypotheses

### H1: Mouse handler uses obsolete split-pane model [CONFIRMED for Bug 2]
- Bead: cclv-5ur.48.1
- Status: CONFIRMED as root cause of mouse bug (Bug 2)
- NOT the cause of vertical text (Bug 1)

### H2: Keyboard expand routes to wrong ConversationViewState [ELIMINATED]
- Bead: cclv-5ur.48.2
- Status: CLOSED
- Reason: expand_handler.rs uses identical selected_tab routing to scroll_handler.rs

### H3: Vertical text caused by viewport_width=0 [LEADING - CONFIRMED for Bug 1]
- Bead: cclv-5ur.48.3
- Status: CONFIRMED as root cause of vertical text (Bug 1)
- Refined: width=0, not width=1

### H4: Subagent entries never relayout after expand toggle [SUBSUMED by H3]
- Bead: cclv-5ur.48.4
- Status: Merged into H3 - relayout is missing at creation time, not just at expand

## Evidence Log

| ID | Source | Finding | Supports | Refutes |
|----|--------|---------|----------|---------|
| E1 | layout.rs:124-125 | calculate_pane_areas returns None for subagent | H1 | - |
| E2 | mouse_handler.rs:125-177 | Subagent detection is dead code | H1 | - |
| E3 | Git history | 3 identical routing bugs fixed before | H2 (but H2 was wrong) | - |
| E4 | Git historian | 15-line routing logic duplicated in 4+ handlers | Architecture debt | - |
| E5 | expand_handler.rs:37-65 | Uses correct selected_tab routing | - | H2 |
| E6 | conversation.rs:108 | viewport_width: 0 in constructor | H3 | - |
| E7 | Test snapshot | Keyboard Enter causes vertical text | H3 | H1 for Bug 1 |
| E8 | session.rs:83-96 | subagent_mut creates without relayout | H3 | - |

## Dead Ends

- **H2**: Keyboard expand routing is correct (identical to fixed scroll_handler)
- Initial suspicion of routing bugs in expand_handler was wrong - that was already fixed

## Recommended Fixes

### Fix 1: Bug 1 - Vertical Text (Priority: P1)
**Location**: `view_state/session.rs`

Option A: Store viewport_width in SessionViewState and call relayout on subagent creation
```rust
pub struct SessionViewState {
    viewport_width: u16,  // Add this
    global_wrap: WrapMode,  // Add this
    // ...
}

pub fn subagent_mut(&mut self, id: &AgentId) -> &mut ConversationViewState {
    if !self.subagents.contains_key(id) {
        let view_state = ConversationViewState::new(...);
        self.subagents.insert(id.clone(), view_state);

        // NEW: Relayout immediately if we have dimensions
        if self.viewport_width > 0 {
            self.subagents.get_mut(id).unwrap().relayout(self.viewport_width, self.global_wrap);
        }
    }
    self.subagents.get_mut(id).unwrap()
}
```

Option B: Relayout ALL subagents in flush_pending_entries() (already done, but check timing)

### Fix 2: Bug 2 - Mouse Clicks (Priority: P1)
**Location**: `state/mouse_handler.rs`

Refactor `detect_entry_click()` to use `selected_tab` routing instead of separate pane areas:
```rust
pub fn detect_entry_click(...) -> EntryClickResult {
    // Use selected_tab to determine which conversation to check
    let selected_tab = state.selected_tab.unwrap_or(0);

    // All conversations render in same area now (unified tabs)
    if !area.contains(click_position) {
        return EntryClickResult::NoEntry;
    }

    let conversation = if selected_tab == 0 {
        state.session_view().main()
    } else {
        // Get subagent by sorted index
        state.session_view().subagent_by_index(selected_tab - 1)?
    };

    // Hit test on the CORRECT conversation
    conversation.hit_test(viewport_y, viewport_x, scroll_offset)
}
```

### Fix 3: Architecture - Central Routing (Priority: P2)
Create `state/routing.rs` with centralized tab→conversation routing to prevent future bugs.

## Next Steps

1. Create fix beads for Bug 1 and Bug 2
2. Implement Fix 1 (viewport_width propagation)
3. Implement Fix 2 (mouse handler refactor)
4. Run test to verify both bugs fixed
5. Close cclv-5ur.48
