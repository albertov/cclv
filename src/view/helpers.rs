//! Helper functions for constructing common Line patterns in the view layer

use ratatui::text::Line;
#[allow(unused_imports)] // Remove after implementation
use ratatui::widgets::{Block, Borders};
#[allow(unused_imports)] // Remove after implementation
use ratatui::style::{Color, Style};
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

/// Creates a Block with focus-based border styling
///
/// Returns a Block with:
/// - The specified title
/// - ALL borders enabled
/// - Yellow border when focused, White when unfocused
///
/// This encapsulates the focus border pattern used across stats, message, and other panels.
#[allow(dead_code)] // Remove after tests written
pub fn styled_block(_title: &str, _focused: bool) -> Block<'_> {
    todo!("styled_block")
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

    // ===== styled_block Tests =====

    #[test]
    fn styled_block_returns_block_when_focused() {
        let block = styled_block("Test", true);

        // Type-level test: function returns a Block
        // If this compiles, the function signature is correct
        let _verify: Block<'_> = block;
    }

    #[test]
    fn styled_block_returns_block_when_unfocused() {
        let block = styled_block("Test", false);

        // Type-level test: function returns a Block
        // If this compiles, the function signature is correct
        let _verify: Block<'_> = block;
    }

    #[test]
    fn styled_block_accepts_different_titles() {
        // Should work with various title strings
        let _block1 = styled_block("", true);
        let _block2 = styled_block("A very long title with many words", false);
        let _block3 = styled_block(" Padded ", true);
    }

    #[test]
    #[should_panic(expected = "styled_block")]
    fn styled_block_focused_fails_on_stub() {
        let _block = styled_block("Test", true);
    }

    #[test]
    #[should_panic(expected = "styled_block")]
    fn styled_block_unfocused_fails_on_stub() {
        let _block = styled_block("Test", false);
    }
}
