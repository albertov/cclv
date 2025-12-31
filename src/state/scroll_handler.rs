//! Vertical scrolling keyboard action handler.
//!
//! Pure functions that transform AppState in response to scroll actions.
//! Focus-aware: dispatches actions to the correct scroll state based on current focus.

use crate::model::KeyAction;
use crate::state::AppState;

/// Handle a scroll keyboard action, dispatching to the appropriate scroll state.
///
/// # Arguments
/// * `state` - Current application state to transform
/// * `action` - The scroll action to handle
/// * `viewport_height` - Height of the visible viewport (for page scrolling)
///
/// Returns a new AppState with the scroll action applied.
pub fn handle_scroll_action(
    _state: AppState,
    _action: KeyAction,
    _viewport_height: usize,
) -> AppState {
    todo!("handle_scroll_action")
}

// ===== Tests =====

#[cfg(test)]
#[path = "scroll_handler_tests.rs"]
mod tests;
