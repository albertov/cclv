//! Message expand/collapse keyboard action handler.
//!
//! Pure functions that transform AppState in response to expand/collapse actions.
//! Focus-aware: dispatches actions to the correct scroll state based on current focus.

use crate::model::KeyAction;
use crate::state::{AppState, FocusPane};

/// Handle a message expand/collapse keyboard action.
///
/// # Arguments
/// * `state` - Current application state to transform
/// * `action` - The expand/collapse action to handle
///
/// Returns a new AppState with the expand/collapse action applied.
pub fn handle_expand_action(mut state: AppState, action: KeyAction) -> AppState {
    // Early return for non-expandable panes
    match state.focus {
        FocusPane::Stats | FocusPane::Search => return state,
        _ => {}
    }

    // Get mutable reference to the appropriate scroll state and conversation
    let (scroll_state, entries) = match state.focus {
        FocusPane::Main => {
            let entries: Vec<_> = state
                .session()
                .main_agent()
                .entries()
                .iter()
                .filter_map(|e| e.as_valid().map(|log| log.uuid().clone()))
                .collect();
            (&mut state.main_scroll, entries)
        }
        FocusPane::Subagent => {
            // Get the currently selected subagent's entries
            let entries = if let Some(tab_index) = state.selected_tab {
                let subagent_ids = state.session().subagent_ids_ordered();
                if let Some(&agent_id) = subagent_ids.get(tab_index) {
                    if let Some(conv) = state.session().subagents().get(agent_id) {
                        conv.entries()
                            .iter()
                            .filter_map(|e| e.as_valid().map(|log| log.uuid().clone()))
                            .collect()
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };
            (&mut state.subagent_scroll, entries)
        }
        _ => return state, // Already handled above
    };

    // Apply the action
    match action {
        KeyAction::ToggleExpand => {
            // Toggle the focused message
            if let Some(focused_idx) = scroll_state.focused_message() {
                if let Some(uuid) = entries.get(focused_idx) {
                    scroll_state.toggle_expand(uuid);
                }
            }
        }
        KeyAction::ExpandMessage => {
            // Expand all messages in current pane
            scroll_state.expand_all(entries.into_iter());
        }
        KeyAction::CollapseMessage => {
            // Collapse all messages in current pane
            scroll_state.collapse_all();
        }
        // Non-expand actions are no-ops
        _ => {}
    }

    state
}

// ===== Tests =====

#[cfg(test)]
#[path = "expand_handler_tests.rs"]
mod tests;
