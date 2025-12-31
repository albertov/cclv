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
