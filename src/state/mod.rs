//! UI state machine (pure).
//!
//! All state transitions are pure functions testable without TUI.

pub mod app_state;
pub mod scroll_handler;
pub mod search;

// Re-export for convenience
pub use app_state::{AppState, FocusPane, ScrollState};
pub use scroll_handler::handle_scroll_action;
pub use search::{SearchMatch, SearchQuery, SearchState};
