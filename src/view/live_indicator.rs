//! LIVE indicator widget for status bar.
//!
//! Displays streaming status per FR-042b:
//! - Gray when Static or Eof
//! - Blinking green when Streaming

#![allow(unused_imports, dead_code)] // Temporary during stub phase

use super::styles::MUTED_TEXT;
use crate::state::InputMode;
use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// Text content for the LIVE indicator.
const LIVE_INDICATOR_PREFIX: &str = "[LIVE] ";

/// LIVE indicator widget that renders based on InputMode and blink state.
///
/// # Functional Requirement
///
/// **FR-042b**: System MUST display a "LIVE" indicator in the status bar:
/// gray when static mode or after EOF, blinking green when actively streaming from stdin.
///
/// # Design
///
/// This widget is pure and stateless. It accepts:
/// - `input_mode`: Current input mode (Static, Streaming, Eof)
/// - `blink_on`: Whether the blink animation is currently ON (managed externally by timer)
///
/// The blink state is passed in rather than managed internally, following the
/// principle of separating state management from rendering.
///
/// # Examples
///
/// ```rust
/// use cclv::view::live_indicator::LiveIndicator;
/// use cclv::state::InputMode;
///
/// // Static mode - always gray
/// let indicator = LiveIndicator::new(InputMode::Static, false, true);
///
/// // Streaming mode - blinking green (when tailing enabled)
/// let indicator_visible = LiveIndicator::new(InputMode::Streaming, true, true);
/// let indicator_hidden = LiveIndicator::new(InputMode::Streaming, false, true);
///
/// // EOF - always gray
/// let indicator_eof = LiveIndicator::new(InputMode::Eof, true, true);
///
/// // Streaming but tailing disabled (viewing historical session) - hidden
/// let indicator_historical = LiveIndicator::new(InputMode::Streaming, true, false);
/// ```
#[derive(Debug, Clone)]
pub struct LiveIndicator {
    mode: InputMode,
    blink_on: bool,
    tailing_enabled: bool,
}

impl LiveIndicator {
    /// Create a new LiveIndicator with the given mode, blink state, and tailing state.
    ///
    /// # Arguments
    ///
    /// * `mode` - The current input mode (Static, Streaming, or Eof)
    /// * `blink_on` - Whether the blink animation is currently ON (only relevant for Streaming mode)
    /// * `tailing_enabled` - Whether live tailing is enabled (viewing last session with auto_scroll)
    pub fn new(mode: InputMode, blink_on: bool, tailing_enabled: bool) -> Self {
        Self {
            mode,
            blink_on,
            tailing_enabled,
        }
    }

    /// Render the indicator as a ratatui Span.
    ///
    /// # Behavior
    ///
    /// - `InputMode::Static` → Gray "[LIVE]" text (regardless of tailing_enabled)
    /// - `InputMode::Eof` → Gray "[LIVE]" text (regardless of tailing_enabled)
    /// - `InputMode::Streaming` with `tailing_enabled=false` → Hidden (viewing historical session)
    /// - `InputMode::Streaming` with `tailing_enabled=true` and `blink_on=true` → Green "[LIVE]" text
    /// - `InputMode::Streaming` with `tailing_enabled=true` and `blink_on=false` → Empty string (hidden)
    ///
    /// # Returns
    ///
    /// A `Span` containing the styled indicator text.
    pub fn render(&self) -> Span<'static> {
        todo!("LiveIndicator::render with tailing_enabled support")
    }
}

// ===== Tests =====

#[cfg(test)]
#[path = "live_indicator_tests.rs"]
mod tests;
