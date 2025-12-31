//! Search input handling (pure state transitions).
//!
//! Handles text input for the SearchState::Typing variant.
//! All functions are pure - no side effects, testable without TUI.

use crate::state::SearchState;

/// Handle character input when in Typing state.
/// Inserts the character at cursor position and advances cursor.
///
/// Returns updated SearchState. No-op if not in Typing state.
pub fn handle_char_input(_state: SearchState, _ch: char) -> SearchState {
    todo!("handle_char_input")
}

/// Handle backspace when in Typing state.
/// Deletes character before cursor if cursor > 0.
///
/// Returns updated SearchState. No-op if not in Typing state.
pub fn handle_backspace(_state: SearchState) -> SearchState {
    todo!("handle_backspace")
}

/// Move cursor left by one position.
/// Saturates at 0 (does not wrap).
///
/// Returns updated SearchState. No-op if not in Typing state.
pub fn handle_cursor_left(_state: SearchState) -> SearchState {
    todo!("handle_cursor_left")
}

/// Move cursor right by one position.
/// Saturates at query length (does not wrap).
///
/// Returns updated SearchState. No-op if not in Typing state.
pub fn handle_cursor_right(_state: SearchState) -> SearchState {
    todo!("handle_cursor_right")
}

/// Activate search input mode.
/// Transitions from Inactive to Typing with empty query and cursor at 0.
///
/// No-op if already in Typing or Active state.
pub fn activate_search_input(_state: SearchState) -> SearchState {
    todo!("activate_search_input")
}

/// Cancel search input.
/// Transitions from Typing or Active to Inactive.
///
/// No-op if already Inactive.
pub fn cancel_search(_state: SearchState) -> SearchState {
    todo!("cancel_search")
}

/// Submit search query.
/// Transitions from Typing to Active if query is non-empty.
/// If query is empty, transitions to Inactive instead.
///
/// Returns updated SearchState. No-op if not in Typing state.
/// Note: Actual search execution happens elsewhere - this just changes state.
pub fn submit_search(_state: SearchState) -> SearchState {
    todo!("submit_search")
}

// ===== Tests =====

#[cfg(test)]
#[path = "search_input_handler_tests.rs"]
mod tests;
