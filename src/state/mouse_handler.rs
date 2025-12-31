//! Mouse event handler.
//!
//! Pure functions that transform AppState in response to mouse events.

use crate::model::AgentId;
use crate::state::AppState;

/// Result of detecting which tab was clicked.
///
/// The tab bar needs to expose its layout (tab positions) so we can
/// map click coordinates to tab indices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabClickResult {
    /// Click was on tab at index
    TabClicked(usize),
    /// Click was outside any tab
    NoTab,
}

/// Detect which tab (if any) was clicked based on mouse position.
///
/// # Arguments
/// * `click_x` - Mouse click column position (0-based)
/// * `click_y` - Mouse click row position (0-based)
/// * `tab_area` - The rectangular area containing the tab bar
/// * `agent_ids` - Ordered list of agent IDs (determines tab count and labels)
///
/// # Returns
/// * `TabClickResult::TabClicked(index)` - Click was on tab at index
/// * `TabClickResult::NoTab` - Click was outside any tab
///
/// # Behavior
/// - Returns NoTab if click is outside tab_area bounds
/// - Calculates tab widths based on agent_id lengths and available space
/// - Returns the index of the clicked tab if within bounds
pub fn detect_tab_click(
    _click_x: u16,
    _click_y: u16,
    _tab_area: ratatui::layout::Rect,
    _agent_ids: &[&AgentId],
) -> TabClickResult {
    todo!("detect_tab_click")
}

/// Handle a mouse click event and update AppState accordingly.
///
/// # Arguments
/// * `state` - Current application state
/// * `click_x` - Mouse click column position
/// * `click_y` - Mouse click row position
/// * `tab_area` - The rectangular area containing the tab bar
///
/// # Returns
/// Updated AppState with tab selection changed if a tab was clicked.
///
/// # Behavior
/// - If click is on a tab, switches to that tab (updates selected_tab)
/// - If click is outside tabs, state is unchanged
/// - Uses agent_ids from state.session() to determine tab layout
pub fn handle_mouse_click(
    _state: AppState,
    _click_x: u16,
    _click_y: u16,
    _tab_area: ratatui::layout::Rect,
) -> AppState {
    todo!("handle_mouse_click")
}

// ===== Tests =====

#[cfg(test)]
#[path = "mouse_handler_tests.rs"]
mod tests;
