//! Log pane widget for displaying internal tracing logs.

use crate::state::LogPaneEntry;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::collections::VecDeque;

// ===== LogPaneView Widget =====

/// Log pane widget for rendering tracing log entries.
///
/// Displays log entries in a scrollable pane with:
/// - Timestamp (HH:MM:SS format)
/// - Color-coded level (ERROR=Red, WARN=Yellow, INFO=Cyan, DEBUG=Gray, TRACE=DarkGray)
/// - Message text
/// - Border showing "Logs" or "Logs [N new]" for unread count
pub struct LogPaneView<'a> {
    /// Reference to log entries (oldest to newest)
    entries: &'a VecDeque<LogPaneEntry>,
    /// Number of unread entries (for title indicator)
    unread_count: usize,
    /// Vertical scroll offset (0 = showing newest entries)
    scroll_offset: usize,
    /// Whether this pane has focus (affects border color)
    focused: bool,
}

impl<'a> LogPaneView<'a> {
    /// Create a new LogPaneView widget.
    ///
    /// # Arguments
    /// * `entries` - Reference to log entries (oldest to newest in VecDeque)
    /// * `unread_count` - Number of unread entries for title indicator
    /// * `scroll_offset` - Vertical scroll position (0 = showing newest)
    /// * `focused` - Whether this pane has focus (Yellow border if focused, White otherwise)
    pub fn new(
        entries: &'a VecDeque<LogPaneEntry>,
        unread_count: usize,
        scroll_offset: usize,
        focused: bool,
    ) -> Self {
        Self {
            entries,
            unread_count,
            scroll_offset,
            focused,
        }
    }
}

impl<'a> Widget for LogPaneView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Build title with unread indicator
        let title = if self.unread_count > 0 {
            format!(" Logs [{} new] ", self.unread_count)
        } else {
            " Logs ".to_string()
        };

        // Border color based on focus
        let border_color = if self.focused {
            Color::Yellow
        } else {
            Color::White
        };

        // Create block with border
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        let inner = block.inner(area);
        block.render(area, buf);

        // Build lines for log entries (newest first)
        let mut lines = Vec::new();
        let available_height = inner.height as usize;

        // Get entries to display (newest first, accounting for scroll offset)
        let entries_vec: Vec<_> = self.entries.iter().collect();
        let total_entries = entries_vec.len();

        if total_entries > 0 {
            // Calculate which entries to show
            // scroll_offset=0 means showing newest entries
            // scroll_offset=N means we've scrolled up by N entries from bottom
            let start_idx = total_entries.saturating_sub(available_height + self.scroll_offset);
            let end_idx = total_entries.saturating_sub(self.scroll_offset);

            for entry in entries_vec[start_idx..end_idx].iter().rev() {
                let timestamp = format_timestamp(&entry.timestamp);
                let level_str = level_to_string(entry.level);
                let level_style = style_for_level(entry.level);

                // Format: [LEVEL] HH:MM:SS | message
                // Use owned Strings in Spans to avoid lifetime issues
                let line = Line::from(vec![
                    ratatui::text::Span::styled(level_str, level_style),
                    ratatui::text::Span::raw(" ".to_string()),
                    ratatui::text::Span::raw(timestamp),
                    ratatui::text::Span::raw(" | ".to_string()),
                    ratatui::text::Span::raw(entry.message.clone()),
                ]);

                lines.push(line);
            }
        }

        // Render paragraph
        let paragraph = Paragraph::new(lines);
        paragraph.render(inner, buf);
    }
}

/// Get the style for a log level.
///
/// Returns color-coded style:
/// - ERROR: Red
/// - WARN: Yellow
/// - INFO: Cyan
/// - DEBUG: Gray (DarkGray)
/// - TRACE: DarkGray (dimmed further)
fn style_for_level(level: tracing::Level) -> Style {
    use tracing::Level;

    let color = match level {
        Level::ERROR => Color::Red,
        Level::WARN => Color::Yellow,
        Level::INFO => Color::Cyan,
        Level::DEBUG => Color::DarkGray,
        Level::TRACE => Color::DarkGray,
    };

    Style::default().fg(color)
}

/// Format timestamp as HH:MM:SS.
///
/// Extracts time portion from a UTC datetime.
fn format_timestamp(timestamp: &chrono::DateTime<chrono::Utc>) -> String {
    timestamp.format("%H:%M:%S").to_string()
}

/// Convert log level to 5-character string.
///
/// Returns level name right-padded to 5 characters:
/// - ERROR
/// - WARN
/// - INFO
/// - DEBUG
/// - TRACE
fn level_to_string(level: tracing::Level) -> String {
    use tracing::Level;

    let name = match level {
        Level::ERROR => "ERROR",
        Level::WARN => "WARN",
        Level::INFO => "INFO",
        Level::DEBUG => "DEBUG",
        Level::TRACE => "TRACE",
    };

    format!("{:5}", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use ratatui::style::Color;
    use std::collections::VecDeque;
    use tracing::Level;

    // ===== Helper: Create test LogPaneEntry =====

    fn create_test_entry(level: Level, message: &str) -> LogPaneEntry {
        LogPaneEntry {
            timestamp: Utc::now(),
            level,
            message: message.to_string(),
        }
    }

    // ===== Helper: Extract text from buffer =====

    fn buffer_to_string(buffer: &Buffer) -> String {
        let area = buffer.area();
        let mut lines = Vec::new();

        for y in area.top()..area.bottom() {
            let mut line = String::new();
            for x in area.left()..area.right() {
                let cell = &buffer[(x, y)];
                line.push_str(cell.symbol());
            }
            let trimmed = line.trim_end();
            if !trimmed.is_empty() {
                lines.push(trimmed.to_string());
            }
        }

        lines.join("\n")
    }

    // ===== LogPaneView::new tests =====

    #[test]
    fn log_pane_view_new_creates_widget() {
        let entries = VecDeque::new();
        let widget = LogPaneView::new(&entries, 0, 0, false);

        // Type-level test: widget exists
        let _verify: LogPaneView = widget;
    }

    // ===== Empty state rendering tests =====

    #[test]
    fn log_pane_view_renders_empty_state() {
        let entries = VecDeque::new();
        let widget = LogPaneView::new(&entries, 0, 0, false);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 40, 10));
        widget.render(Rect::new(0, 0, 40, 10), &mut buffer);

        let content = buffer_to_string(&buffer);

        // Should have "Logs" title in border
        assert!(
            content.contains("Logs"),
            "Expected 'Logs' title in empty state, got:\n{}",
            content
        );
    }

    // ===== Entry formatting tests =====

    #[test]
    fn log_pane_view_displays_single_entry_with_timestamp() {
        let mut entries = VecDeque::new();
        let entry = create_test_entry(Level::INFO, "Test message");
        entries.push_back(entry);

        let widget = LogPaneView::new(&entries, 0, 0, false);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 60, 10));
        widget.render(Rect::new(0, 0, 60, 10), &mut buffer);

        let content = buffer_to_string(&buffer);

        // Should contain timestamp in HH:MM:SS format
        // We can't assert exact time, but we can check format pattern
        assert!(
            content.contains(":") && content.chars().filter(|&c| c == ':').count() >= 2,
            "Expected timestamp with HH:MM:SS format (contains at least 2 colons), got:\n{}",
            content
        );

        // Should contain message text
        assert!(
            content.contains("Test message"),
            "Expected message text 'Test message', got:\n{}",
            content
        );
    }

    #[test]
    fn log_pane_view_displays_entry_with_level_label() {
        let mut entries = VecDeque::new();
        let entry = create_test_entry(Level::ERROR, "Error occurred");
        entries.push_back(entry);

        let widget = LogPaneView::new(&entries, 0, 0, false);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 60, 10));
        widget.render(Rect::new(0, 0, 60, 10), &mut buffer);

        let content = buffer_to_string(&buffer);

        // Should contain level indicator (ERROR, WARN, etc.)
        assert!(
            content.contains("ERROR") || content.contains("ERR"),
            "Expected level label 'ERROR' or 'ERR', got:\n{}",
            content
        );

        assert!(
            content.contains("Error occurred"),
            "Expected message 'Error occurred', got:\n{}",
            content
        );
    }

    // ===== Level color tests =====

    #[test]
    fn log_pane_view_applies_red_color_for_error_level() {
        let mut entries = VecDeque::new();
        let entry = create_test_entry(Level::ERROR, "Error message");
        entries.push_back(entry);

        let widget = LogPaneView::new(&entries, 0, 0, false);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 60, 10));
        widget.render(Rect::new(0, 0, 60, 10), &mut buffer);

        // Find cells containing "ERROR" or "ERR" and verify they have red color
        let area = buffer.area();
        let mut found_red_level = false;

        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let cell = &buffer[(x, y)];
                let symbol = cell.symbol();
                if (symbol.contains('E') || symbol.contains('R')) && cell.fg == Color::Red {
                    found_red_level = true;
                    break;
                }
            }
        }

        assert!(
            found_red_level,
            "Expected ERROR level to be rendered with red foreground color"
        );
    }

    #[test]
    fn log_pane_view_applies_yellow_color_for_warn_level() {
        let mut entries = VecDeque::new();
        let entry = create_test_entry(Level::WARN, "Warning message");
        entries.push_back(entry);

        let widget = LogPaneView::new(&entries, 0, 0, false);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 60, 10));
        widget.render(Rect::new(0, 0, 60, 10), &mut buffer);

        let area = buffer.area();
        let mut found_yellow = false;

        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let cell = &buffer[(x, y)];
                if cell.fg == Color::Yellow {
                    found_yellow = true;
                    break;
                }
            }
        }

        assert!(
            found_yellow,
            "Expected WARN level to have yellow foreground color somewhere in output"
        );
    }

    #[test]
    fn log_pane_view_applies_cyan_color_for_info_level() {
        let mut entries = VecDeque::new();
        let entry = create_test_entry(Level::INFO, "Info message");
        entries.push_back(entry);

        let widget = LogPaneView::new(&entries, 0, 0, false);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 60, 10));
        widget.render(Rect::new(0, 0, 60, 10), &mut buffer);

        let area = buffer.area();
        let mut found_cyan = false;

        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let cell = &buffer[(x, y)];
                if cell.fg == Color::Cyan {
                    found_cyan = true;
                    break;
                }
            }
        }

        assert!(
            found_cyan,
            "Expected INFO level to have cyan foreground color somewhere in output"
        );
    }

    #[test]
    fn log_pane_view_applies_darkgray_color_for_debug_level() {
        let mut entries = VecDeque::new();
        let entry = create_test_entry(Level::DEBUG, "Debug message");
        entries.push_back(entry);

        let widget = LogPaneView::new(&entries, 0, 0, false);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 60, 10));
        widget.render(Rect::new(0, 0, 60, 10), &mut buffer);

        let area = buffer.area();
        let mut found_darkgray = false;

        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let cell = &buffer[(x, y)];
                if cell.fg == Color::DarkGray || cell.fg == Color::Gray {
                    found_darkgray = true;
                    break;
                }
            }
        }

        assert!(
            found_darkgray,
            "Expected DEBUG level to have DarkGray or Gray foreground color"
        );
    }

    #[test]
    fn log_pane_view_applies_darkgray_color_for_trace_level() {
        let mut entries = VecDeque::new();
        let entry = create_test_entry(Level::TRACE, "Trace message");
        entries.push_back(entry);

        let widget = LogPaneView::new(&entries, 0, 0, false);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 60, 10));
        widget.render(Rect::new(0, 0, 60, 10), &mut buffer);

        let area = buffer.area();
        let mut found_darkgray = false;

        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let cell = &buffer[(x, y)];
                if cell.fg == Color::DarkGray {
                    found_darkgray = true;
                    break;
                }
            }
        }

        assert!(
            found_darkgray,
            "Expected TRACE level to have DarkGray foreground color"
        );
    }

    // ===== Focus state tests =====

    #[test]
    fn log_pane_view_unfocused_has_white_border() {
        let entries = VecDeque::new();
        let widget = LogPaneView::new(&entries, 0, 0, false); // not focused

        let mut buffer = Buffer::empty(Rect::new(0, 0, 40, 10));
        widget.render(Rect::new(0, 0, 40, 10), &mut buffer);

        // Check border cells for white color
        let area = buffer.area();
        let mut found_white_border = false;

        // Check top border row
        for x in area.left()..area.right() {
            let cell = &buffer[(x, area.top())];
            if cell.fg == Color::White {
                found_white_border = true;
                break;
            }
        }

        assert!(
            found_white_border,
            "Expected unfocused border to have white color"
        );
    }

    #[test]
    fn log_pane_view_focused_has_yellow_border() {
        let entries = VecDeque::new();
        let widget = LogPaneView::new(&entries, 0, 0, true); // focused

        let mut buffer = Buffer::empty(Rect::new(0, 0, 40, 10));
        widget.render(Rect::new(0, 0, 40, 10), &mut buffer);

        let area = buffer.area();
        let mut found_yellow_border = false;

        // Check top border row
        for x in area.left()..area.right() {
            let cell = &buffer[(x, area.top())];
            if cell.fg == Color::Yellow {
                found_yellow_border = true;
                break;
            }
        }

        assert!(
            found_yellow_border,
            "Expected focused border to have yellow color"
        );
    }

    // ===== Unread count indicator tests =====

    #[test]
    fn log_pane_view_shows_unread_count_when_nonzero() {
        let entries = VecDeque::new();
        let widget = LogPaneView::new(&entries, 5, 0, false); // 5 unread

        let mut buffer = Buffer::empty(Rect::new(0, 0, 40, 10));
        widget.render(Rect::new(0, 0, 40, 10), &mut buffer);

        let content = buffer_to_string(&buffer);

        // Should show "[5 new]" or similar in title
        assert!(
            content.contains("5") || content.contains("new"),
            "Expected unread count '5' or 'new' in title, got:\n{}",
            content
        );
    }

    #[test]
    fn log_pane_view_hides_unread_indicator_when_zero() {
        let entries = VecDeque::new();
        let widget = LogPaneView::new(&entries, 0, 0, false); // 0 unread

        let mut buffer = Buffer::empty(Rect::new(0, 0, 40, 10));
        widget.render(Rect::new(0, 0, 40, 10), &mut buffer);

        let content = buffer_to_string(&buffer);

        // Should show just "Logs", not "[N new]"
        assert!(
            content.contains("Logs"),
            "Expected 'Logs' title, got:\n{}",
            content
        );

        // Should NOT contain unread indicator
        assert!(
            !content.contains("new"),
            "Expected no 'new' indicator when unread_count is 0, got:\n{}",
            content
        );
    }

    // ===== Scroll offset tests =====

    #[test]
    fn log_pane_view_displays_newest_entries_when_scroll_offset_zero() {
        let mut entries = VecDeque::new();
        entries.push_back(create_test_entry(Level::INFO, "Old message"));
        entries.push_back(create_test_entry(Level::INFO, "Newer message"));
        entries.push_back(create_test_entry(Level::INFO, "Newest message"));

        let widget = LogPaneView::new(&entries, 0, 0, false); // scroll_offset = 0

        let mut buffer = Buffer::empty(Rect::new(0, 0, 60, 10));
        widget.render(Rect::new(0, 0, 60, 10), &mut buffer);

        let content = buffer_to_string(&buffer);

        // With scroll_offset=0, newest should be visible
        assert!(
            content.contains("Newest message"),
            "Expected 'Newest message' to be visible with scroll_offset=0, got:\n{}",
            content
        );
    }

    #[test]
    fn log_pane_view_scrolls_with_nonzero_offset() {
        // Create more entries than fit in the viewport
        let mut entries = VecDeque::new();
        for i in 0..20 {
            entries.push_back(create_test_entry(Level::INFO, &format!("Message {}", i)));
        }

        // scroll_offset > 0 means we're scrolled up (away from newest)
        let widget = LogPaneView::new(&entries, 0, 10, false); // scrolled up by 10

        let mut buffer = Buffer::empty(Rect::new(0, 0, 60, 10));
        widget.render(Rect::new(0, 0, 60, 10), &mut buffer);

        let content = buffer_to_string(&buffer);

        // Newest message (19) should NOT be visible when scrolled up
        assert!(
            !content.contains("Message 19"),
            "Expected newest message to NOT be visible when scrolled up, got:\n{}",
            content
        );

        // Older messages should be visible
        // (Exact message depends on implementation, but we know newest shouldn't show)
    }

    // ===== Helper function tests =====

    #[test]
    fn format_timestamp_returns_hh_mm_ss_format() {
        use chrono::{TimeZone, Utc};

        let timestamp = Utc.with_ymd_and_hms(2024, 1, 15, 14, 32, 45).unwrap();
        let formatted = format_timestamp(&timestamp);

        assert_eq!(
            formatted, "14:32:45",
            "Expected timestamp formatted as HH:MM:SS"
        );
    }

    #[test]
    fn style_for_level_error_returns_red() {
        let style = style_for_level(Level::ERROR);
        assert_eq!(
            style.fg,
            Some(Color::Red),
            "ERROR level should have red foreground"
        );
    }

    #[test]
    fn style_for_level_warn_returns_yellow() {
        let style = style_for_level(Level::WARN);
        assert_eq!(
            style.fg,
            Some(Color::Yellow),
            "WARN level should have yellow foreground"
        );
    }

    #[test]
    fn style_for_level_info_returns_cyan() {
        let style = style_for_level(Level::INFO);
        assert_eq!(
            style.fg,
            Some(Color::Cyan),
            "INFO level should have cyan foreground"
        );
    }

    #[test]
    fn style_for_level_debug_returns_darkgray_or_gray() {
        let style = style_for_level(Level::DEBUG);
        assert!(
            style.fg == Some(Color::DarkGray) || style.fg == Some(Color::Gray),
            "DEBUG level should have DarkGray or Gray foreground, got {:?}",
            style.fg
        );
    }

    #[test]
    fn style_for_level_trace_returns_darkgray() {
        let style = style_for_level(Level::TRACE);
        assert_eq!(
            style.fg,
            Some(Color::DarkGray),
            "TRACE level should have DarkGray foreground"
        );
    }
}
