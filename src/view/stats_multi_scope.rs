//! Multi-scope statistics panel widget.
//!
//! Displays three stat scopes simultaneously:
//! 1. Focused conversation (currently selected tab - main or subagent)
//! 2. Session totals (all agents in current session combined)
//! 3. Global totals (all sessions - future multi-session support)

use super::helpers::empty_line;
use super::styles::SECTION_HEADER;
use crate::model::{PricingConfig, SessionStats, StatsFilter};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

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
    stats: &'a SessionStats,

    /// Filter for the currently focused conversation.
    /// Determines what appears in the "Focused" column.
    focused_filter: &'a StatsFilter,

    /// Pricing configuration for cost estimation.
    pricing: &'a PricingConfig,

    /// Model ID for pricing lookup (defaults to "opus" if None).
    model_id: Option<&'a str>,

    /// Whether this panel currently has focus (affects border color).
    focused: bool,
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
        stats: &'a SessionStats,
        focused_filter: &'a StatsFilter,
        pricing: &'a PricingConfig,
        model_id: Option<&'a str>,
        focused: bool,
    ) -> Self {
        Self {
            stats,
            focused_filter,
            pricing,
            model_id,
            focused,
        }
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
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Border style based on focus
        let border_color = if self.focused {
            Color::Yellow
        } else {
            Color::White
        };

        let block = Block::default()
            .title(" Multi-Scope Statistics ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        let inner = block.inner(area);
        block.render(area, buf);

        // Split into three columns if width permits, otherwise stack vertically
        // Minimum width for side-by-side: ~90 chars (30 per scope)
        let use_columns = inner.width >= 90;

        if use_columns {
            // Three columns side-by-side
            let columns = Layout::horizontal([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(inner);

            // Render each scope
            render_stat_scope(
                columns[0],
                buf,
                self.stats,
                self.focused_filter,
                self.pricing,
                self.model_id,
                scope_title(self.focused_filter),
            );

            render_stat_scope(
                columns[1],
                buf,
                self.stats,
                &StatsFilter::Global,
                self.pricing,
                self.model_id,
                "Session",
            );

            render_stat_scope(
                columns[2],
                buf,
                self.stats,
                &StatsFilter::Global,
                self.pricing,
                self.model_id,
                "Global",
            );
        } else {
            // Three rows stacked vertically
            let rows = Layout::vertical([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(inner);

            render_stat_scope(
                rows[0],
                buf,
                self.stats,
                self.focused_filter,
                self.pricing,
                self.model_id,
                scope_title(self.focused_filter),
            );

            render_stat_scope(
                rows[1],
                buf,
                self.stats,
                &StatsFilter::Global,
                self.pricing,
                self.model_id,
                "Session",
            );

            render_stat_scope(
                rows[2],
                buf,
                self.stats,
                &StatsFilter::Global,
                self.pricing,
                self.model_id,
                "Global",
            );
        }
    }
}

// ===== Helper Functions =====

/// Render a single stat scope within a bounded area.
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
    area: Rect,
    buf: &mut Buffer,
    stats: &SessionStats,
    filter: &StatsFilter,
    pricing: &PricingConfig,
    model_id: Option<&str>,
    title: &str,
) {
    // Create block for this scope
    let block = Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = block.inner(area);
    block.render(area, buf);

    // Build content lines
    let mut lines = Vec::new();

    // Get filtered usage
    let usage = stats.filtered_usage(filter);

    // Token section (compact)
    lines.push(Line::from("Tokens:").style(SECTION_HEADER));
    lines.push(Line::from(format!(
        "  In:  {}",
        format_tokens(usage.total_input())
    )));
    lines.push(Line::from(format!(
        "  Out: {}",
        format_tokens(usage.output_tokens)
    )));
    lines.push(empty_line());

    // Cost section
    let cost = calculate_cost(&usage, pricing, model_id);
    lines.push(Line::from("Cost:").style(SECTION_HEADER));
    lines.push(Line::from(format!("  {}", format_cost(cost))));
    lines.push(empty_line());

    // Top 5 tools (compact)
    let tool_counts = stats.filtered_tool_counts(filter);
    if !tool_counts.is_empty() {
        lines.push(Line::from("Tools:").style(SECTION_HEADER));
        let tool_lines = format_tool_breakdown(tool_counts, 5);
        lines.extend(tool_lines);
    }

    // Render paragraph
    let paragraph = Paragraph::new(lines);
    paragraph.render(inner, buf);
}

/// Format a scope title based on the filter.
///
/// # Examples
/// - `StatsFilter::Global` → "Session"
/// - `StatsFilter::MainAgent` → "Main Agent"
/// - `StatsFilter::Subagent(_)` → "Subagent"
fn scope_title(filter: &StatsFilter) -> &'static str {
    match filter {
        StatsFilter::Global => "Session",
        StatsFilter::MainAgent => "Main Agent",
        StatsFilter::Subagent(_) => "Subagent",
    }
}

/// Calculate estimated cost in USD for the given token usage.
fn calculate_cost(
    usage: &crate::model::TokenUsage,
    pricing: &PricingConfig,
    model_id: Option<&str>,
) -> f64 {
    let model_pricing = pricing.get(model_id.unwrap_or("opus"));

    let input_cost =
        (usage.input_tokens as f64 / 1_000_000.0) * model_pricing.input_cost_per_million;

    let output_cost =
        (usage.output_tokens as f64 / 1_000_000.0) * model_pricing.output_cost_per_million;

    let cache_rate = model_pricing
        .cached_input_cost_per_million
        .unwrap_or(model_pricing.input_cost_per_million);

    let cache_cost = ((usage.cache_creation_input_tokens + usage.cache_read_input_tokens) as f64
        / 1_000_000.0)
        * cache_rate;

    input_cost + output_cost + cache_cost
}

/// Format tool usage breakdown with top N limiting.
fn format_tool_breakdown(
    tool_counts: &std::collections::HashMap<crate::model::ToolName, u32>,
    max_display: usize,
) -> Vec<Line<'static>> {
    if tool_counts.is_empty() {
        return vec![];
    }

    let mut tools: Vec<_> = tool_counts.iter().collect();
    tools.sort_by(|a, b| b.1.cmp(a.1));

    let mut lines = Vec::new();
    let total_tools = tools.len();

    for (tool_name, count) in tools.iter().take(max_display) {
        lines.push(Line::from(format!("  {}: {}", tool_name.as_str(), count)));
    }

    if total_tools > max_display {
        let remaining = total_tools - max_display;
        lines.push(Line::from(format!("  ... and {} more", remaining)));
    }

    lines
}

/// Format a token count with thousands separators.
fn format_tokens(tokens: u64) -> String {
    let s = tokens.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }

    result
}

/// Format a cost value in USD.
fn format_cost(cost: f64) -> String {
    let rounded = (cost * 100.0).round() / 100.0;
    let dollars = rounded.floor() as u64;
    let cents = ((rounded - dollars as f64) * 100.0).round() as u64;
    let dollars_str = format_tokens(dollars);
    format!("${}.{:02}", dollars_str, cents)
}
