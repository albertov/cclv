//! Match navigation handler.
//!
//! Pure functions for navigating between search matches.
//! Handles next/prev navigation with wrap-around and focus/tab switching.

use crate::model::AgentId;
use crate::state::AppState;

// ===== Public API =====

/// Navigate to the next search match.
///
/// Behavior:
/// - If not in Active search state, does nothing
/// - Increments current_match by 1
/// - Wraps from last match to first (0)
/// - Switches focus to Main or Subagent pane based on match location
/// - Selects correct subagent tab if match is in a subagent
pub fn next_match(_state: AppState) -> AppState {
    todo!("next_match")
}

/// Navigate to the previous search match.
///
/// Behavior:
/// - If not in Active search state, does nothing
/// - Decrements current_match by 1
/// - Wraps from first match (0) to last
/// - Switches focus to Main or Subagent pane based on match location
/// - Selects correct subagent tab if match is in a subagent
pub fn prev_match(_state: AppState) -> AppState {
    todo!("prev_match")
}

// ===== Helper Functions =====

/// Find the tab index for a given agent_id.
/// Returns None if agent_id is not found in subagents.
fn find_tab_for_agent(_state: &AppState, _agent_id: &AgentId) -> Option<usize> {
    todo!("find_tab_for_agent")
}

/// Switch focus and tab to the correct location for a search match.
/// If agent_id is None, switches to Main pane.
/// If agent_id is Some, switches to Subagent pane and selects the correct tab.
fn switch_to_match_location(
    _state: AppState,
    _agent_id: &Option<AgentId>,
) -> AppState {
    todo!("switch_to_match_location")
}

// ===== Tests =====

#[cfg(test)]
#[path = "match_navigation_handler_tests.rs"]
mod tests;
