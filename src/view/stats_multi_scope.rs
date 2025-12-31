//! Multi-scope statistics panel widget.
//!
//! Displays three stat scopes simultaneously:
//! 1. Focused conversation (currently selected tab - main or subagent)
//! 2. Session totals (all agents in current session combined)
//! 3. Global totals (all sessions - future multi-session support)

use crate::model::{PricingConfig, SessionStats, StatsFilter};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

// ===== MultiScopeStatsPanel Widget =====

/// Multi-scope statistics panel widget.
///
/// Displays three stat scopes side-by-side or stacked:
/// - **Focused**: Stats for the currently focused conversation (main or selected subagent tab)
/// - **Session**: Totals for the current session (all agents combined)
/// - **Global**: Grand totals across all sessions (when multi-session logs loaded)
///
/// # Design
///
/// The widget computes three `StatsFilter` scopes internally:
/// 1. `focused_filter` - passed in, reflects current tab selection
/// 2. `StatsFilter::Global` - always shows session-wide totals
/// 3. Future: multi-session global (currently same as session)
///
/// # Layout
///
/// Renders three columns (or stacked rows if width constrained):
/// ```text
/// ┌─ Focused ──┬─ Session ──┬─ Global ───┐
/// │ Input: 100 │ Input: 500 │ Input: 500 │
/// │ ...        │ ...        │ ...        │
/// └────────────┴────────────┴────────────┘
/// ```
pub struct MultiScopeStatsPanel<'a> {
    /// Session statistics (source of truth for all scopes).
    _stats: &'a SessionStats,

    /// Filter for the currently focused conversation.
    /// Determines what appears in the "Focused" column.
    _focused_filter: &'a StatsFilter,

    /// Pricing configuration for cost estimation.
    _pricing: &'a PricingConfig,

    /// Model ID for pricing lookup (defaults to "opus" if None).
    _model_id: Option<&'a str>,

    /// Whether this panel currently has focus (affects border color).
    _focused: bool,
}

impl<'a> MultiScopeStatsPanel<'a> {
    /// Create a new MultiScopeStatsPanel widget.
    ///
    /// # Arguments
    /// * `stats` - Session statistics (contains all data for all scopes)
    /// * `focused_filter` - Filter for currently focused conversation (main or subagent tab)
    /// * `pricing` - Pricing configuration for cost estimation
    /// * `model_id` - Model ID for pricing lookup (defaults to "opus" if None)
    /// * `focused` - Whether this panel currently has focus (affects border color)
    ///
    /// # Returns
    /// New `MultiScopeStatsPanel` widget ready to render.
    pub fn new(
        _stats: &'a SessionStats,
        _focused_filter: &'a StatsFilter,
        _pricing: &'a PricingConfig,
        _model_id: Option<&'a str>,
        _focused: bool,
    ) -> Self {
        todo!("MultiScopeStatsPanel::new")
    }
}

impl<'a> Widget for MultiScopeStatsPanel<'a> {
    /// Render the multi-scope stats panel.
    ///
    /// Displays three stat scopes:
    /// 1. Focused conversation stats (left/top)
    /// 2. Session totals (middle)
    /// 3. Global totals (right/bottom - currently same as session)
    ///
    /// Layout adapts to available width:
    /// - Wide: three columns side-by-side
    /// - Narrow: three rows stacked vertically
    fn render(self, _area: Rect, _buf: &mut Buffer) {
        todo!("MultiScopeStatsPanel::render")
    }
}

// ===== Helper Functions =====

/// Render a single stat scope within a bounded area.
#[allow(dead_code)]
///
/// # Arguments
/// * `area` - Rectangle to render within
/// * `buf` - Buffer to render to
/// * `stats` - Session statistics
/// * `filter` - Which scope to display (Focused/Session/Global)
/// * `pricing` - Pricing configuration
/// * `model_id` - Model ID for cost calculation
/// * `title` - Title for this scope (e.g., "Focused", "Session", "Global")
///
/// # Design
///
/// This is extracted from the original `StatsPanel` rendering logic.
/// Renders a compact summary of tokens, cost, and top tools.
fn render_stat_scope(
    _area: Rect,
    _buf: &mut Buffer,
    _stats: &SessionStats,
    _filter: &StatsFilter,
    _pricing: &PricingConfig,
    _model_id: Option<&str>,
    _title: &str,
) {
    todo!("render_stat_scope")
}

/// Format a scope title based on the filter.
///
/// # Examples
/// - `StatsFilter::Global` → "Session"
/// - `StatsFilter::MainAgent` → "Main Agent"
/// - `StatsFilter::Subagent(_)` → "Subagent"
#[allow(dead_code)]
fn scope_title(_filter: &StatsFilter) -> &'static str {
    todo!("scope_title")
}
