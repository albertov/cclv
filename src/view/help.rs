//! Help overlay widget displaying keyboard shortcuts.
//!
//! Shows a centered modal overlay with all keyboard shortcuts grouped by category.
//! Triggered by '?' key, dismissed by 'Esc' or '?'.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

/// Render the help overlay centered on the screen.
///
/// The overlay displays all keyboard shortcuts grouped by category:
/// - Navigation
/// - Pane Focus
/// - Tabs (Subagent Pane)
/// - Message Interaction
/// - Search
/// - Stats
/// - Live Mode
/// - Application
///
/// The overlay is centered on the screen with a border and dismissal hint.
pub fn render_help_overlay(frame: &mut Frame) {
    todo!("render_help_overlay")
}

/// Calculate the centered rect for the help overlay.
///
/// Returns a Rect that is centered on the screen with the specified
/// percentage of width and height.
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    todo!("centered_rect")
}

/// Build the help content lines grouped by category.
///
/// Returns a Vec of Line representing all shortcuts with category headers.
fn build_help_content() -> Vec<Line<'static>> {
    todo!("build_help_content")
}

// ===== Tests =====

#[cfg(test)]
#[path = "help_tests.rs"]
mod tests;
