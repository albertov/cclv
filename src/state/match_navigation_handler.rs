//! Match navigation handler.
//!
//! Pure functions for navigating between search matches.
//! Handles next/prev navigation with wrap-around and focus/tab switching.

use crate::model::{AgentId, EntryUuid};
use crate::state::{AppState, FocusPane, SearchState};
use crate::view_state::scroll::ScrollPosition;
use crate::view_state::types::EntryIndex;

// ===== Public API =====

/// Navigate to the next search match.
///
/// Behavior:
/// - If not in Active search state, does nothing
/// - Increments current_match by 1
/// - Wraps from last match to first (0)
/// - Switches focus to Main or Subagent pane based on match location
/// - Selects correct subagent tab if match is in a subagent
pub fn next_match(mut state: AppState) -> AppState {
    // Only operate when in Active search state
    if let SearchState::Active {
        query,
        matches,
        current_match,
    } = &state.search
    {
        // Cannot navigate if no matches
        if matches.is_empty() {
            return state;
        }

        // Calculate next match index with wrap-around
        let next_index = if *current_match + 1 >= matches.len() {
            0 // Wrap to first
        } else {
            current_match + 1
        };

        // Clone data we need before mutating state
        let target_agent_id = matches[next_index].agent_id.clone();
        let target_entry_uuid = matches[next_index].entry_uuid.clone();
        let query = query.clone();
        let matches = matches.clone();

        // Update search state with new current_match
        state.search = SearchState::Active {
            query,
            matches,
            current_match: next_index,
        };

        // Switch focus/tab to match location and scroll to match
        switch_to_match_location(state, &target_agent_id, &target_entry_uuid)
    } else {
        // Not in Active state - do nothing
        state
    }
}

/// Navigate to the previous search match.
///
/// Behavior:
/// - If not in Active search state, does nothing
/// - Decrements current_match by 1
/// - Wraps from first match (0) to last
/// - Switches focus to Main or Subagent pane based on match location
/// - Selects correct subagent tab if match is in a subagent
pub fn prev_match(mut state: AppState) -> AppState {
    // Only operate when in Active search state
    if let SearchState::Active {
        query,
        matches,
        current_match,
    } = &state.search
    {
        // Cannot navigate if no matches
        if matches.is_empty() {
            return state;
        }

        // Calculate previous match index with wrap-around
        let prev_index = if *current_match == 0 {
            matches.len() - 1 // Wrap to last
        } else {
            current_match - 1
        };

        // Clone data we need before mutating state
        let target_agent_id = matches[prev_index].agent_id.clone();
        let target_entry_uuid = matches[prev_index].entry_uuid.clone();
        let query = query.clone();
        let matches = matches.clone();

        // Update search state with new current_match
        state.search = SearchState::Active {
            query,
            matches,
            current_match: prev_index,
        };

        // Switch focus/tab to match location and scroll to match
        switch_to_match_location(state, &target_agent_id, &target_entry_uuid)
    } else {
        // Not in Active state - do nothing
        state
    }
}

// ===== Helper Functions =====

/// Switch focus and tab to the correct location for a search match.
/// If agent_id is None, switches to Main pane.
/// If agent_id is Some, switches to Subagent pane and selects the correct tab.
/// Also scrolls the conversation to show the match entry (US5/FR-013).
fn switch_to_match_location(
    mut state: AppState,
    agent_id: &Option<AgentId>,
    entry_uuid: &EntryUuid,
) -> AppState {
    match agent_id {
        None => {
            // Match is in main agent - switch to Main pane
            state.focus = FocusPane::Main;
        }
        Some(aid) => {
            // Match is in subagent - switch to Subagent pane and select conversation
            state.focus = FocusPane::Subagent;

            // Select the subagent conversation by AgentId (cclv-5ur.53)
            state.selected_conversation =
                crate::state::ConversationSelection::Subagent(aid.clone());
        }
    }

    // Scroll to the match entry (US5/FR-013)
    scroll_to_entry(&mut state, entry_uuid);

    state
}

/// Scroll the selected conversation to show the entry with the given UUID.
/// Does nothing if the entry is not found or no conversation is selected.
fn scroll_to_entry(state: &mut AppState, entry_uuid: &EntryUuid) {
    // Get mutable reference to selected conversation
    if let Some(conversation) = state.selected_conversation_view_mut() {
        // Find the entry index for this UUID
        if let Some(entry_index) = find_entry_index_by_uuid(conversation, entry_uuid) {
            // Set scroll position to show this entry
            conversation.set_scroll(ScrollPosition::AtEntry {
                entry_index,
                line_in_entry: 0, // Scroll to top of entry
            });
        }
    }
}

/// Find the entry index for a given UUID in a conversation.
/// Returns None if the UUID is not found.
fn find_entry_index_by_uuid(
    conversation: &crate::view_state::conversation::ConversationViewState,
    target_uuid: &EntryUuid,
) -> Option<EntryIndex> {
    conversation
        .iter()
        .enumerate()
        .find(|(_, entry_view)| {
            entry_view
                .uuid()
                .map(|uuid| uuid == target_uuid)
                .unwrap_or(false)
        })
        .map(|(idx, _)| EntryIndex::new(idx))
}

// ===== Tests =====

#[cfg(test)]
#[path = "match_navigation_handler_tests.rs"]
mod tests;
