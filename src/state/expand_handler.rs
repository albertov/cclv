//! Message expand/collapse keyboard action handler.
//!
//! Pure functions that transform AppState in response to expand/collapse actions.
//! Tab-aware: dispatches actions to the correct ConversationViewState based on selected_tab.

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
///
/// # Routing Logic
///
/// Routes to the appropriate conversation view based on selected_tab:
/// - Tab 0 = Main agent conversation
/// - Tab 1+ = Subagent conversations (index - 1 in sorted subagent list)
///
/// This matches the routing in scroll_handler.rs to ensure consistency.
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

    // Route based on selected_tab (matches scroll_handler.rs routing logic)
    let selected_tab_index = state.selected_tab.unwrap_or(0);

    let conversation = if selected_tab_index == 0 {
        // Tab 0: Main agent conversation
        if let Some(session) = state.log_view_mut().current_session_mut() {
            session.main_mut()
        } else {
            return state; // No session, nothing to expand/collapse
        }
    } else {
        // Tabs 1+: Subagent conversation (index - 1 in subagent list)
        let subagent_index = selected_tab_index - 1;

        // Get agent ID at subagent_index and clone to avoid borrow conflicts
        let mut agent_ids: Vec<_> = state.session_view().subagent_ids().cloned().collect();
        agent_ids.sort_by(|a, b| a.as_str().cmp(b.as_str()));
        let agent_id = agent_ids.get(subagent_index).cloned();

        if let Some(agent_id) = agent_id {
            if let Some(session) = state.log_view_mut().current_session_mut() {
                session.subagent_mut(&agent_id)
            } else {
                return state;
            }
        } else {
            return state; // Subagent not found
        }
    };

    // Apply the action to the selected conversation view
    match action {
        KeyAction::ToggleExpand => {
            // Toggle the focused message, or entry 0 if no message is focused
            let idx_to_toggle = conversation
                .focused_message()
                .unwrap_or(EntryIndex::new(0));

            // Only toggle if the entry exists
            if idx_to_toggle.get() < conversation.len() {
                conversation.toggle_entry_expanded(idx_to_toggle.get());
            }
        }
        KeyAction::ExpandMessage => {
            // Expand all messages in current pane
            let count = conversation.len();
            for i in 0..count {
                if let Some(entry) = conversation.get(EntryIndex::new(i)) {
                    if !entry.is_expanded() {
                        conversation.toggle_entry_expanded(i);
                    }
                }
            }
        }
        KeyAction::CollapseMessage => {
            // Collapse all messages in current pane
            let count = conversation.len();
            for i in 0..count {
                if let Some(entry) = conversation.get(EntryIndex::new(i)) {
                    if entry.is_expanded() {
                        conversation.toggle_entry_expanded(i);
                    }
                }
            }
        }
        _ => {}
    }

    state
}

// ===== Tests =====

#[cfg(test)]
#[path = "expand_handler_tests.rs"]
mod tests;
