//! Tests for compute_entry_lines unified renderer.

use super::compute_entry_lines;
use crate::model::{ContentBlock, ConversationEntry, EntryMetadata, EntryType, LogEntry, Message, MessageContent, Role};
use crate::state::WrapMode;
use crate::model::identifiers::{EntryUuid, SessionId};
use chrono::Utc;

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
    );

    let nowrap_lines = compute_entry_lines(
        &entry,
        true, // expanded
        WrapMode::NoWrap,
        width,
        collapse_threshold,
        summary_lines,
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
