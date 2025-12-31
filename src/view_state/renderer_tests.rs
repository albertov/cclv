//! Tests for compute_entry_lines unified renderer.

use super::compute_entry_lines;
use crate::model::{ContentBlock, ConversationEntry, EntryMetadata, EntryType, LogEntry, Message, MessageContent, Role};
use crate::state::WrapMode;
use crate::model::identifiers::{EntryUuid, SessionId};
use crate::view::MessageStyles;
use chrono::Utc;

/// Helper to create default MessageStyles for tests.
fn default_styles() -> MessageStyles {
    MessageStyles::new()
}

// ===== Role-Based Styling Tests (FR-021, FR-022) =====

#[test]
fn test_user_entry_has_cyan_color() {
    // Create a User entry with simple text
    let text = "User message";
    let entry = create_entry_with_text(text);

    // Render with MessageStyles
    let styles = default_styles();
    let lines = compute_entry_lines(
        &entry,
        false, // collapsed
        WrapMode::Wrap,
        80,
        10,
        3,
        &styles,
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // FR-021: User messages should have Cyan color
    // Check that at least one line has Cyan foreground color
    let has_cyan = lines.iter().any(|line| {
        line.spans.iter().any(|span| {
            span.style.fg == Some(ratatui::style::Color::Cyan)
        })
    });

    assert!(
        has_cyan,
        "User entry should have at least one span with Cyan color (FR-021)"
    );
}

#[test]
fn test_assistant_entry_has_green_color() {
    // Create an Assistant entry with Thinking block
    let thinking_text = "Assistant thinking...";
    let entry = create_entry_with_thinking(thinking_text);

    // Render with MessageStyles
    let styles = default_styles();
    let lines = compute_entry_lines(
        &entry,
        false, // collapsed
        WrapMode::Wrap,
        80,
        10,
        3,
        &styles,
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // FR-022: Assistant messages should have Green color
    // Check that at least one line has Green foreground color
    let has_green = lines.iter().any(|line| {
        line.spans.iter().any(|span| {
            span.style.fg == Some(ratatui::style::Color::Green)
        })
    });

    assert!(
        has_green,
        "Assistant entry should have at least one span with Green color (FR-022)"
    );
}

/// Helper to create a test LogEntry with Thinking block.
fn create_entry_with_thinking(thinking_text: &str) -> ConversationEntry {
    let thinking_lines = thinking_text.lines().count();
    eprintln!("Creating entry with {} lines of thinking content", thinking_lines);

    let blocks = vec![ContentBlock::Thinking {
        thinking: thinking_text.to_string(),
    }];

    let message = Message::new(Role::Assistant, MessageContent::Blocks(blocks));
    let uuid = EntryUuid::new("test-uuid-001").unwrap();
    let session_id = SessionId::new("test-session").unwrap();
    let timestamp = Utc::now();

    let log_entry = LogEntry::new(
        uuid,
        None, // parent_uuid
        session_id,
        None, // agent_id
        timestamp,
        EntryType::Assistant,
        message,
        EntryMetadata::default(),
    );
    ConversationEntry::Valid(Box::new(log_entry))
}

#[test]
fn test_collapsed_thinking_block_respects_collapse_threshold() {
    // Create entry with 100 lines of Thinking content
    let thinking_text = (0..100).map(|i| format!("Thinking line {}", i)).collect::<Vec<_>>().join("\n");
    let entry = create_entry_with_thinking(&thinking_text);

    let collapse_threshold = 10;
    let summary_lines = 3;

    // Render collapsed
    let lines = compute_entry_lines(
        &entry,
        false, // expanded = false
        WrapMode::Wrap,
        80,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // ASSERTION: Collapsed Thinking block should show:
    // - 3 summary lines (first 3 lines of Thinking content)
    // - 1 collapse indicator: "(+97 more lines)"
    // - 1 separator line (blank line at end)
    // Total: 5 lines
    //
    // This is the KEY fix: Currently message.rs renders all 100 lines of Thinking
    // because Thinking blocks never collapse there, but height calculator counts
    // them as 4 lines (collapsed). This test ensures they collapse consistently.
    assert_eq!(
        lines.len(),
        5,
        "Collapsed Thinking block should show {} summary + 1 indicator + 1 separator = 5 lines, got {}",
        summary_lines,
        lines.len()
    );

    // Verify collapse indicator is present
    let has_collapse_indicator = lines.iter().any(|line: &ratatui::text::Line<'static>| {
        // Check if any span contains "more lines"
        line.spans.iter().any(|span| span.content.contains("more lines"))
    });
    assert!(
        has_collapse_indicator,
        "Collapsed entry should include '(+N more lines)' indicator"
    );
}

#[test]
fn test_expanded_thinking_block_shows_all_lines() {
    // Create entry with 100 lines of Thinking content
    let thinking_text = (0..100).map(|i| format!("Thinking line {}", i)).collect::<Vec<_>>().join("\n");
    let entry = create_entry_with_thinking(&thinking_text);

    let collapse_threshold = 10;
    let summary_lines = 3;

    // Render expanded
    let lines = compute_entry_lines(
        &entry,
        true, // expanded = true
        WrapMode::Wrap,
        80,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // ASSERTION: Expanded Thinking block should show all 100 lines + 1 separator
    // Total: 101 lines
    assert_eq!(
        lines.len(),
        101,
        "Expanded Thinking block should show all 100 content lines + 1 separator = 101 lines, got {}",
        lines.len()
    );

    // Verify NO collapse indicator
    let has_collapse_indicator = lines.iter().any(|line: &ratatui::text::Line<'static>| {
        line.spans.iter().any(|span| span.content.contains("more lines"))
    });
    assert!(
        !has_collapse_indicator,
        "Expanded entry should NOT include collapse indicator"
    );
}

#[test]
fn test_small_thinking_block_never_collapses() {
    // Create entry with 5 lines of Thinking content (below threshold)
    let thinking_text = (0..5).map(|i| format!("Thinking line {}", i)).collect::<Vec<_>>().join("\n");
    let entry = create_entry_with_thinking(&thinking_text);

    let collapse_threshold = 10;
    let summary_lines = 3;

    // Render collapsed (but should show all since below threshold)
    let lines = compute_entry_lines(
        &entry,
        false, // expanded = false
        WrapMode::Wrap,
        80,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // ASSERTION: Below-threshold entry shows all lines even when "collapsed"
    // - 5 lines of Thinking content
    // - 1 separator line
    // Total: 6 lines
    assert_eq!(
        lines.len(),
        6,
        "Below-threshold entry should show all 5 lines + 1 separator = 6 lines, got {}",
        lines.len()
    );

    // Verify NO collapse indicator
    let has_collapse_indicator = lines.iter().any(|line: &ratatui::text::Line<'static>| {
        line.spans.iter().any(|span| span.content.contains("more lines"))
    });
    assert!(
        !has_collapse_indicator,
        "Below-threshold entry should NOT include collapse indicator"
    );
}

/// Helper to create a test LogEntry with simple text content.
fn create_entry_with_text(text: &str) -> ConversationEntry {
    let message = Message::new(Role::User, MessageContent::Text(text.to_string()));
    let uuid = EntryUuid::new("test-text-001").unwrap();
    let session_id = SessionId::new("test-session").unwrap();
    let timestamp = Utc::now();

    let log_entry = LogEntry::new(
        uuid,
        None, // parent_uuid
        session_id,
        None, // agent_id
        timestamp,
        EntryType::User,
        message,
        EntryMetadata::default(),
    );
    ConversationEntry::Valid(Box::new(log_entry))
}

#[test]
fn test_collapsed_text_content_respects_collapse_threshold() {
    // Create entry with 100 lines of text content
    let text = (0..100).map(|i| format!("Text line {}", i)).collect::<Vec<_>>().join("\n");
    let entry = create_entry_with_text(&text);

    let collapse_threshold = 10;
    let summary_lines = 3;

    // Render collapsed
    let lines = compute_entry_lines(
        &entry,
        false, // expanded = false
        WrapMode::Wrap,
        80,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // ASSERTION: Collapsed Text content should show:
    // - 3 summary lines (first 3 lines of text)
    // - 1 collapse indicator: "(+97 more lines)"
    // - 1 separator line (blank line at end)
    // Total: 5 lines
    assert_eq!(
        lines.len(),
        5,
        "Collapsed Text content should show {} summary + 1 indicator + 1 separator = 5 lines, got {}",
        summary_lines,
        lines.len()
    );

    // Verify collapse indicator is present
    let has_collapse_indicator = lines.iter().any(|line: &ratatui::text::Line<'static>| {
        line.spans.iter().any(|span| span.content.contains("more lines"))
    });
    assert!(
        has_collapse_indicator,
        "Collapsed text entry should include '(+N more lines)' indicator"
    );

    // Verify first line contains actual text content
    let first_line_text = lines[0].spans.iter()
        .map(|span| span.content.as_ref())
        .collect::<String>();
    assert!(
        first_line_text.contains("Text line 0"),
        "First line should contain 'Text line 0', got: '{}'",
        first_line_text
    );
}

#[test]
fn test_expanded_text_content_shows_all_lines() {
    // Create entry with 100 lines of text content
    let text = (0..100).map(|i| format!("Text line {}", i)).collect::<Vec<_>>().join("\n");
    let entry = create_entry_with_text(&text);

    let collapse_threshold = 10;
    let summary_lines = 3;

    // Render expanded
    let lines = compute_entry_lines(
        &entry,
        true, // expanded = true
        WrapMode::Wrap,
        80,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // ASSERTION: Expanded Text content should show all 100 lines + 1 separator
    // Total: 101 lines
    assert_eq!(
        lines.len(),
        101,
        "Expanded Text content should show all 100 content lines + 1 separator = 101 lines, got {}",
        lines.len()
    );

    // Verify NO collapse indicator
    let has_collapse_indicator = lines.iter().any(|line: &ratatui::text::Line<'static>| {
        line.spans.iter().any(|span| span.content.contains("more lines"))
    });
    assert!(
        !has_collapse_indicator,
        "Expanded text entry should NOT include collapse indicator"
    );
}

#[test]
fn test_small_text_content_never_collapses() {
    // Create entry with 5 lines of text content (below threshold)
    let text = (0..5).map(|i| format!("Text line {}", i)).collect::<Vec<_>>().join("\n");
    let entry = create_entry_with_text(&text);

    let collapse_threshold = 10;
    let summary_lines = 3;

    // Render collapsed (but should show all since below threshold)
    let lines = compute_entry_lines(
        &entry,
        false, // expanded = false
        WrapMode::Wrap,
        80,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // ASSERTION: Below-threshold text entry shows all lines even when "collapsed"
    // - 5 lines of text content
    // - 1 separator line
    // Total: 6 lines
    assert_eq!(
        lines.len(),
        6,
        "Below-threshold text entry should show all 5 lines + 1 separator = 6 lines, got {}",
        lines.len()
    );

    // Verify NO collapse indicator
    let has_collapse_indicator = lines.iter().any(|line: &ratatui::text::Line<'static>| {
        line.spans.iter().any(|span| span.content.contains("more lines"))
    });
    assert!(
        !has_collapse_indicator,
        "Below-threshold text entry should NOT include collapse indicator"
    );
}

// ============================================================================
// WRAPPING TESTS - Test that all content block types wrap consistently
// ============================================================================

#[test]
fn test_text_block_wraps_long_lines() {
    // Create entry with a single very long line (100 chars)
    let long_line = "x".repeat(100);
    let entry = create_entry_with_text(&long_line);

    let width = 40; // Narrow viewport
    let collapse_threshold = 10;
    let summary_lines = 3;

    // Render with wrapping enabled
    let lines = compute_entry_lines(
        &entry,
        true, // expanded
        WrapMode::Wrap,
        width,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // ASSERTION: With content_width = 40 - 2 = 38 chars, a 100-char line
    // should wrap to ceil(100/38) = 3 lines, plus 1 separator = 4 total
    //
    // This test ensures Text blocks apply wrap_lines() like Thinking blocks do.
    assert_eq!(
        lines.len(),
        4,
        "100-char line should wrap to 3 lines + 1 separator = 4 lines at width {}, got {}",
        width,
        lines.len()
    );
}

#[test]
fn test_text_block_nowrap_does_not_wrap() {
    // Create entry with a single very long line (100 chars)
    let long_line = "x".repeat(100);
    let entry = create_entry_with_text(&long_line);

    let width = 40; // Narrow viewport
    let collapse_threshold = 10;
    let summary_lines = 3;

    // Render with NoWrap mode
    let lines = compute_entry_lines(
        &entry,
        true, // expanded
        WrapMode::NoWrap,
        width,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // ASSERTION: NoWrap mode keeps the 100-char line as a single line
    // 1 content line + 1 separator = 2 total
    assert_eq!(
        lines.len(),
        2,
        "NoWrap mode should keep long line unwrapped: 1 line + 1 separator = 2 lines, got {}",
        lines.len()
    );
}

/// Helper to create a test LogEntry with ToolResult content block.
fn create_entry_with_tool_result(content: &str, is_error: bool) -> ConversationEntry {
    use crate::model::ToolUseId;

    let blocks = vec![ContentBlock::ToolResult {
        tool_use_id: ToolUseId::new("test-tool-use-001").unwrap(),
        content: content.to_string(),
        is_error,
    }];

    let message = Message::new(Role::User, MessageContent::Blocks(blocks));
    let uuid = EntryUuid::new("test-tool-result-001").unwrap();
    let session_id = SessionId::new("test-session").unwrap();
    let timestamp = Utc::now();

    let log_entry = LogEntry::new(
        uuid,
        None, // parent_uuid
        session_id,
        None, // agent_id
        timestamp,
        EntryType::User,
        message,
        EntryMetadata::default(),
    );
    ConversationEntry::Valid(Box::new(log_entry))
}

#[test]
fn test_tool_result_wraps_long_lines() {
    // Create entry with a single very long line (100 chars) in ToolResult
    let long_line = "y".repeat(100);
    let entry = create_entry_with_tool_result(&long_line, false);

    let width = 40; // Narrow viewport
    let collapse_threshold = 10;
    let summary_lines = 3;

    // Render with wrapping enabled
    let lines = compute_entry_lines(
        &entry,
        true, // expanded
        WrapMode::Wrap,
        width,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // ASSERTION: With content_width = 40 - 2 = 38 chars, a 100-char line
    // should wrap to ceil(100/38) = 3 lines, plus 1 separator = 4 total
    //
    // This test ensures ToolResult blocks apply wrap_lines() like Thinking blocks do.
    assert_eq!(
        lines.len(),
        4,
        "100-char ToolResult line should wrap to 3 lines + 1 separator = 4 lines at width {}, got {}",
        width,
        lines.len()
    );
}

#[test]
fn test_tool_result_nowrap_does_not_wrap() {
    // Create entry with a single very long line (100 chars)
    let long_line = "y".repeat(100);
    let entry = create_entry_with_tool_result(&long_line, false);

    let width = 40; // Narrow viewport
    let collapse_threshold = 10;
    let summary_lines = 3;

    // Render with NoWrap mode
    let lines = compute_entry_lines(
        &entry,
        true, // expanded
        WrapMode::NoWrap,
        width,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // ASSERTION: NoWrap mode keeps the 100-char line as a single line
    // 1 content line + 1 separator = 2 total
    assert_eq!(
        lines.len(),
        2,
        "NoWrap mode should keep long ToolResult unwrapped: 1 line + 1 separator = 2 lines, got {}",
        lines.len()
    );
}

/// Helper to create a test LogEntry with ToolUse content block.
fn create_entry_with_tool_use(tool_name: &str, input_json: serde_json::Value) -> ConversationEntry {
    use crate::model::{ToolCall, ToolName, ToolUseId};

    let tool_call = ToolCall::new(
        ToolUseId::new("test-tool-use-002").unwrap(),
        ToolName::parse(tool_name),
        input_json,
    );

    let blocks = vec![ContentBlock::ToolUse(tool_call)];

    let message = Message::new(Role::Assistant, MessageContent::Blocks(blocks));
    let uuid = EntryUuid::new("test-tool-use-002").unwrap();
    let session_id = SessionId::new("test-session").unwrap();
    let timestamp = Utc::now();

    let log_entry = LogEntry::new(
        uuid,
        None, // parent_uuid
        session_id,
        None, // agent_id
        timestamp,
        EntryType::Assistant,
        message,
        EntryMetadata::default(),
    );
    ConversationEntry::Valid(Box::new(log_entry))
}

#[test]
fn test_tool_use_wraps_long_input_lines() {
    // Create entry with ToolUse that has a long string value
    let long_value = "z".repeat(100);
    let input = serde_json::json!({
        "long_param": long_value
    });
    let entry = create_entry_with_tool_use("TestTool", input);

    let width = 40; // Narrow viewport
    let collapse_threshold = 10;
    let summary_lines = 3;

    // Render with wrapping enabled
    let lines = compute_entry_lines(
        &entry,
        true, // expanded
        WrapMode::Wrap,
        width,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // ASSERTION: ToolUse renders as:
    // - 1 header line: "Tool: TestTool"
    // - N input lines (pretty-printed JSON with long string that should wrap)
    // - 1 separator
    //
    // The JSON line with the 100-char string should wrap to multiple lines.
    // We expect MORE than 3 lines total (header + wrapped JSON + separator)
    assert!(
        lines.len() > 3,
        "ToolUse with 100-char parameter should wrap to >3 lines at width {}, got {}",
        width,
        lines.len()
    );
}

#[test]
fn test_tool_use_nowrap_does_not_wrap() {
    // Create entry with ToolUse that has a long string value
    let long_value = "z".repeat(100);
    let input = serde_json::json!({
        "long_param": long_value
    });
    let entry = create_entry_with_tool_use("TestTool", input);

    let width = 40; // Narrow viewport
    let collapse_threshold = 10;
    let summary_lines = 3;

    // Render with NoWrap mode
    let wrapped_lines = compute_entry_lines(
        &entry,
        true, // expanded
        WrapMode::Wrap,
        width,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    let nowrap_lines = compute_entry_lines(
        &entry,
        true, // expanded
        WrapMode::NoWrap,
        width,
        collapse_threshold,
        summary_lines,
        &default_styles(),
        None, // No index prefix for existing tests
        false, // Not a subagent view
    );

    // ASSERTION: NoWrap mode should produce FEWER lines than Wrap mode
    // because long lines stay unwrapped
    assert!(
        nowrap_lines.len() < wrapped_lines.len(),
        "NoWrap mode should produce fewer lines than Wrap mode, got NoWrap={} Wrap={}",
        nowrap_lines.len(),
        wrapped_lines.len()
    );
}

#[test]
fn test_tool_use_header_has_emoji_indicator() {
    // Create entry with simple ToolUse
    let input = serde_json::json!({
        "param": "value"
    });
    let entry = create_entry_with_tool_use("TestTool", input);

    let styles = default_styles();
    let lines = compute_entry_lines(
        &entry,
        true, // expanded
        WrapMode::Wrap,
        80,
        10,
        3,
        &styles,
        None, // No index prefix
        false, // Not a subagent view
    );

    // ASSERTION: First line should be the header with emoji: "🔧 Tool: TestTool"
    let first_line_text: String = lines[0].spans.iter()
        .map(|span| span.content.as_ref())
        .collect();

    assert!(
        first_line_text.starts_with("🔧 Tool:"),
        "ToolUse header should start with '🔧 Tool:', got: '{}'",
        first_line_text
    );

    assert!(
        first_line_text.contains("TestTool"),
        "ToolUse header should contain tool name 'TestTool', got: '{}'",
        first_line_text
    );
}

// ============================================================================
// ENTRY INDEX PREFIX TESTS - Test that entry indices appear as prefixes
// ============================================================================

#[test]
fn test_entry_index_0_shows_as_1_prefix() {
    // Create entry with simple text
    let text = "Test message";
    let entry = create_entry_with_text(text);

    let styles = default_styles();
    let lines = compute_entry_lines(
        &entry,
        false, // collapsed
        WrapMode::Wrap,
        80,
        10,
        3,
        &styles,
        Some(0), // Entry index 0 should display as "   1│"
        false, // Not a subagent view
    );

    // ASSERTION: Every content line should have "   1│" prefix
    // (excluding the separator line at the end)
    let content_lines = &lines[..lines.len() - 1]; // All but last (separator)

    for line in content_lines {
        let line_text: String = line.spans.iter()
            .map(|span| span.content.as_ref())
            .collect();

        assert!(
            line_text.starts_with("   1│"),
            "Line should start with '   1│', got: '{}'",
            line_text
        );
    }

    // Verify the prefix span has DarkGray color and DIM modifier
    let first_span = &lines[0].spans[0];
    assert_eq!(
        first_span.style.fg,
        Some(ratatui::style::Color::DarkGray),
        "Index prefix should be DarkGray"
    );
    assert!(
        first_span.style.add_modifier.contains(ratatui::style::Modifier::DIM),
        "Index prefix should have DIM modifier"
    );
}

#[test]
fn test_entry_index_41_shows_as_42_prefix() {
    // Create entry with simple text
    let text = "Test message";
    let entry = create_entry_with_text(text);

    let styles = default_styles();
    let lines = compute_entry_lines(
        &entry,
        false, // collapsed
        WrapMode::Wrap,
        80,
        10,
        3,
        &styles,
        Some(41), // Entry index 41 should display as "  42│"
        false, // Not a subagent view
    );

    // ASSERTION: Every content line should have "  42│" prefix (right-aligned in 4 chars)
    let content_lines = &lines[..lines.len() - 1]; // All but last (separator)

    for line in content_lines {
        let line_text: String = line.spans.iter()
            .map(|span| span.content.as_ref())
            .collect();

        assert!(
            line_text.starts_with("  42│"),
            "Line should start with '  42│', got: '{}'",
            line_text
        );
    }
}

#[test]
fn test_entry_index_999_shows_as_1000_prefix() {
    // Create entry with simple text
    let text = "Test message";
    let entry = create_entry_with_text(text);

    let styles = default_styles();
    let lines = compute_entry_lines(
        &entry,
        false, // collapsed
        WrapMode::Wrap,
        80,
        10,
        3,
        &styles,
        Some(999), // Entry index 999 should display as "1000│" (4 digits)
        false, // Not a subagent view
    );

    // ASSERTION: Every content line should have "1000│" prefix (right-aligned in 4 chars)
    let content_lines = &lines[..lines.len() - 1]; // All but last (separator)

    for line in content_lines {
        let line_text: String = line.spans.iter()
            .map(|span| span.content.as_ref())
            .collect();

        assert!(
            line_text.starts_with("1000│"),
            "Line should start with '1000│', got: '{}'",
            line_text
        );
    }
}

#[test]
fn test_entry_index_none_shows_no_prefix() {
    // Create entry with simple text
    let text = "Test message";
    let entry = create_entry_with_text(text);

    let styles = default_styles();
    let lines = compute_entry_lines(
        &entry,
        false, // collapsed
        WrapMode::Wrap,
        80,
        10,
        3,
        &styles,
        None, // No index = no prefix
        false, // Not a subagent view
    );

    // ASSERTION: Lines should NOT have index prefix
    let content_lines = &lines[..lines.len() - 1]; // All but last (separator)

    for line in content_lines {
        let line_text: String = line.spans.iter()
            .map(|span| span.content.as_ref())
            .collect();

        assert!(
            !line_text.contains("│"),
            "Line should NOT have '│' separator when entry_index is None, got: '{}'",
            line_text
        );
    }
}

#[test]
fn test_entry_index_prefix_on_multiline_entry() {
    // Create entry with 5 lines of text
    let text = (0..5).map(|i| format!("Line {}", i)).collect::<Vec<_>>().join("\n");
    let entry = create_entry_with_text(&text);

    let styles = default_styles();
    let lines = compute_entry_lines(
        &entry,
        true, // expanded (show all lines)
        WrapMode::Wrap,
        80,
        10,
        3,
        &styles,
        Some(0), // Entry index 0 should display as "   1│"
        false, // Not a subagent view
    );

    // ASSERTION: All 5 content lines should have "   1│" prefix
    // Total lines = 5 content + 1 separator = 6
    assert_eq!(lines.len(), 6, "Should have 5 content lines + 1 separator");

    let content_lines = &lines[..5]; // First 5 lines are content

    for (i, line) in content_lines.iter().enumerate() {
        let line_text: String = line.spans.iter()
            .map(|span| span.content.as_ref())
            .collect();

        assert!(
            line_text.starts_with("   1│"),
            "Line {} should start with '   1│', got: '{}'",
            i,
            line_text
        );
    }
}

// ============================================================================
// INITIAL PROMPT LABEL TESTS - Test "🔷 Initial Prompt" in subagent views
// ============================================================================

#[test]
fn test_initial_prompt_label_appears_for_first_entry_in_subagent_view() {
    // Create entry with simple text
    let text = "Initial prompt text";
    let entry = create_entry_with_text(text);

    let styles = default_styles();
    let lines = compute_entry_lines(
        &entry,
        false, // collapsed
        WrapMode::Wrap,
        80,
        10,
        3,
        &styles,
        Some(0), // First entry (index 0)
        true,    // IS a subagent view
    );

    // ASSERTION: First line should be "🔷 Initial Prompt" with Magenta + Bold
    // Second line should be the actual content with index prefix
    assert!(
        lines.len() >= 2,
        "Should have at least 2 lines (label + content), got {}",
        lines.len()
    );

    // Check first line is the Initial Prompt label
    let first_line_text: String = lines[0]
        .spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect();

    assert!(
        first_line_text.contains("🔷 Initial Prompt"),
        "First line should contain '🔷 Initial Prompt', got: '{}'",
        first_line_text
    );

    // Verify the label has Magenta color and Bold modifier
    let label_span = &lines[0].spans[0];
    assert_eq!(
        label_span.style.fg,
        Some(ratatui::style::Color::Magenta),
        "Initial Prompt label should be Magenta"
    );
    assert!(
        label_span
            .style
            .add_modifier
            .contains(ratatui::style::Modifier::BOLD),
        "Initial Prompt label should be BOLD"
    );
}

#[test]
fn test_initial_prompt_label_does_not_appear_in_main_view() {
    // Create entry with simple text
    let text = "First entry in main view";
    let entry = create_entry_with_text(text);

    let styles = default_styles();
    let lines = compute_entry_lines(
        &entry,
        false, // collapsed
        WrapMode::Wrap,
        80,
        10,
        3,
        &styles,
        Some(0), // First entry (index 0)
        false,   // NOT a subagent view (main view)
    );

    // ASSERTION: Should NOT have Initial Prompt label
    // First line should be the content with index prefix "   1│First entry in main view"
    let first_line_text: String = lines[0]
        .spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect();

    assert!(
        !first_line_text.contains("🔷 Initial Prompt"),
        "Main view should NOT show Initial Prompt label, got: '{}'",
        first_line_text
    );

    assert!(
        first_line_text.starts_with("   1│"),
        "First line should start with entry index prefix, got: '{}'",
        first_line_text
    );
}

#[test]
fn test_initial_prompt_label_only_for_first_entry_in_subagent() {
    // Create entry with simple text
    let text = "Second entry in subagent";
    let entry = create_entry_with_text(text);

    let styles = default_styles();
    let lines = compute_entry_lines(
        &entry,
        false, // collapsed
        WrapMode::Wrap,
        80,
        10,
        3,
        &styles,
        Some(1), // Second entry (index 1, not 0)
        true,    // IS a subagent view
    );

    // ASSERTION: Should NOT have Initial Prompt label (only for index 0)
    let first_line_text: String = lines[0]
        .spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect();

    assert!(
        !first_line_text.contains("🔷 Initial Prompt"),
        "Only first entry (index 0) should show Initial Prompt, got: '{}'",
        first_line_text
    );

    assert!(
        first_line_text.starts_with("   2│"),
        "Second entry should have index prefix '   2│', got: '{}'",
        first_line_text
    );
}

#[test]
fn test_initial_prompt_label_without_entry_index() {
    // Test that Initial Prompt label appears even without entry index prefix
    let text = "Initial prompt without index";
    let entry = create_entry_with_text(text);

    let styles = default_styles();
    let lines = compute_entry_lines(
        &entry,
        false, // collapsed
        WrapMode::Wrap,
        80,
        10,
        3,
        &styles,
        None, // No entry index (but we need entry_index == Some(0) for label!)
        true, // IS a subagent view
    );

    // ASSERTION: Should NOT have Initial Prompt label because entry_index is None
    // The label only appears when entry_index == Some(0)
    let first_line_text: String = lines[0]
        .spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect();

    assert!(
        !first_line_text.contains("🔷 Initial Prompt"),
        "Initial Prompt should only appear when entry_index == Some(0), got: '{}'",
        first_line_text
    );
}

#[test]
fn test_entry_index_prefix_on_collapsed_entry() {
    // Create entry with 100 lines (will collapse)
    let text = (0..100).map(|i| format!("Line {}", i)).collect::<Vec<_>>().join("\n");
    let entry = create_entry_with_text(&text);

    let collapse_threshold = 10;
    let summary_lines = 3;
    let styles = default_styles();

    let lines = compute_entry_lines(
        &entry,
        false, // collapsed
        WrapMode::Wrap,
        80,
        collapse_threshold,
        summary_lines,
        &styles,
        Some(9), // Entry index 9 should display as "  10│"
        false, // Not a subagent view
    );

    // ASSERTION: Should have 3 summary lines + 1 collapse indicator + 1 separator = 5 total
    assert_eq!(lines.len(), 5, "Collapsed entry should have 5 lines");

    // All lines except separator should have "  10│" prefix
    let content_lines = &lines[..4]; // 3 summary + 1 collapse indicator

    for (i, line) in content_lines.iter().enumerate() {
        let line_text: String = line.spans.iter()
            .map(|span| span.content.as_ref())
            .collect();

        assert!(
            line_text.starts_with("  10│"),
            "Line {} should start with '  10│', got: '{}'",
            i,
            line_text
        );
    }
}
