//! Helper functions for constructing common Line patterns in the view layer

use ratatui::text::Line;
use std::fmt::Display;

/// Creates an empty line (commonly used as a separator)
pub fn empty_line() -> Line<'static> {
    Line::from("")
}

/// Creates a key-value line with consistent formatting
///
/// Format: "  {key}: {value}"
pub fn key_value_line(key: &str, value: impl Display) -> Line<'static> {
    Line::from(format!("  {}: {}", key, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_line_creates_empty_string() {
        let line = empty_line();
        // Line::from("") produces 0 spans (ratatui optimizes away empty spans)
        assert_eq!(line.spans.len(), 0);
    }

    #[test]
    fn key_value_line_formats_with_indent_and_colon() {
        let line = key_value_line("Count", 42);
        // Should format as "  Count: 42"
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "  Count: 42");
    }

    #[test]
    fn key_value_line_works_with_string_values() {
        let line = key_value_line("Name", "test");
        // Should format as "  Name: test"
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "  Name: test");
    }

    #[test]
    fn key_value_line_works_with_formatted_strings() {
        let formatted = format!("{} tokens", 1234);
        let line = key_value_line("Total", formatted);
        // Should format as "  Total: 1234 tokens"
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "  Total: 1234 tokens");
    }
}
