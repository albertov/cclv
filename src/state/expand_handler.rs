//! Message expand/collapse keyboard action handler.
//!
//! Pure functions that transform AppState in response to expand/collapse actions.
//! Focus-aware: dispatches actions to the correct ConversationViewState based on current focus.

use crate::model::KeyAction;
use crate::state::{AppState, FocusPane};
use crate::view_state::types::EntryIndex;

/// Handle a message expand/collapse keyboard action.
///
/// # Arguments
/// * `state` - Current application state to transform
/// * `action` - The expand/collapse action to handle
/// * `viewport_width` - Viewport width in characters for layout calculations
///
/// Returns a new AppState with the expand/collapse action applied.
pub fn handle_expand_action(
    mut state: AppState,
    action: KeyAction,
    _viewport_width: u16,
) -> AppState {
    // Early return for non-expandable panes
    match state.focus {
        FocusPane::Stats | FocusPane::Search => return state,
        _ => {}
    }

    // Apply the action based on focus
    match state.focus {
        FocusPane::Main => {
            if let Some(session_view) = state.log_view_mut().current_session_mut() {
                let conv_view = session_view.main_mut();

                match action {
                    KeyAction::ToggleExpand => {
                        // Toggle the focused message via ConversationViewState
                        if let Some(focused_idx) = conv_view.focused_message() {
                            conv_view.toggle_entry_expanded(focused_idx.get());
                        }
                    }
                    KeyAction::ExpandMessage => {
                        // Expand all messages in main pane
                        let count = conv_view.len();
                        for i in 0..count {
                            if let Some(entry) = conv_view.get(EntryIndex::new(i)) {
                                if !entry.is_expanded() {
                                    conv_view.toggle_entry_expanded(i);
                                }
                            }
                        }
                    }
                    KeyAction::CollapseMessage => {
                        // Collapse all messages in main pane
                        let count = conv_view.len();
                        for i in 0..count {
                            if let Some(entry) = conv_view.get(EntryIndex::new(i)) {
                                if entry.is_expanded() {
                                    conv_view.toggle_entry_expanded(i);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        FocusPane::Subagent => {
            // TODO: Implement subagent expand/collapse using ConversationViewState
            // This requires identifying which subagent tab is selected and getting its ConversationViewState
        }
        _ => {}
    }

    state
}

// ===== Tests =====

#[cfg(test)]
#[path = "expand_handler_tests.rs"]
mod tests;
