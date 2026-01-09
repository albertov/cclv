//! Session list modal rendering.

use ratatui::prelude::*;

use crate::state::AppState;

/// Render the session list modal overlay.
///
/// Displays a centered modal with:
/// - Session list with current session marked
/// - Selected row highlighted
/// - Footer with keybinding hints
///
/// Only renders when `state.session_modal.is_visible()` is true.
///
/// # Layout
/// - 60 columns wide, centered horizontally
/// - Height adapts to session count
/// - Clears background before rendering for overlay effect
///
/// # FR-002: Session list modal accessible via keyboard
/// # FR-003: Select session from modal using keyboard navigation
/// # FR-005: Visually indicate currently active session
pub fn render_session_modal(_frame: &mut Frame, _state: &AppState) {
    todo!("render_session_modal: not implemented")
}

#[cfg(test)]
#[path = "session_modal_tests.rs"]
mod tests;
