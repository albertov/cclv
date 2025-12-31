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
    todo!("handle_expand_action")
}

// ===== Tests =====

#[cfg(test)]
#[path = "expand_handler_tests.rs"]
mod tests;
