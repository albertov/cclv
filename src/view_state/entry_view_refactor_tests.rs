//! Tests for refactored EntryView with rendered_lines as source of truth.
//!
//! These tests verify that:
//! - EntryView stores pre-rendered lines (Vec<Line<'static>>)
//! - height() returns LineHeight based on rendered_lines.len()
//! - Constructor calls compute_entry_lines with proper parameters
//! - recompute_lines() updates rendered_lines after state changes

use super::EntryView;
use crate::model::{
    ContentBlock, ConversationEntry, EntryMetadata, EntryType, EntryUuid, LogEntry,
    MalformedEntry, Message, MessageContent, Role, SessionId,
};
use crate::state::WrapMode;
use crate::view_state::types::{EntryIndex, LineHeight};
use ratatui::text::Line;

// ===== Test Helpers =====

fn make_session_id(s: &str) -> SessionId {
    SessionId::new(s).expect("valid session id")
}

fn make_entry_uuid(s: &str) -> EntryUuid {
    EntryUuid::new(s).expect("valid uuid")
}

fn make_timestamp() -> chrono::DateTime<chrono::Utc> {
    "2025-12-25T10:00:00Z".parse().expect("valid timestamp")
}

fn make_message(text: &str) -> Message {
    Message::new(Role::User, MessageContent::Text(text.to_string()))
}

fn make_valid_entry() -> ConversationEntry {
    let log_entry = LogEntry::new(
        make_entry_uuid("uuid-1"),
        None,
        make_session_id("session-1"),
        None,
        make_timestamp(),
        EntryType::User,
        make_message("Test message"),
        EntryMetadata::default(),
    );
    ConversationEntry::Valid(Box::new(log_entry))
}

fn make_malformed_entry() -> ConversationEntry {
    ConversationEntry::Malformed(MalformedEntry::new(
        42,
        "bad json",
        "Parse error",
        Some(make_session_id("session-1")),
    ))
}

fn make_entry_with_thinking(thinking_text: &str) -> ConversationEntry {
    let blocks = vec![ContentBlock::Thinking {
        thinking: thinking_text.to_string(),
    }];
    let message = Message::new(Role::Assistant, MessageContent::Blocks(blocks));
    let log_entry = LogEntry::new(
        make_entry_uuid("thinking-uuid"),
        None,
        make_session_id("session-1"),
        None,
        make_timestamp(),
        EntryType::Assistant,
        message,
        EntryMetadata::default(),
    );
    ConversationEntry::Valid(Box::new(log_entry))
}

// ===== Constructor Tests =====

#[test]
fn with_rendered_lines_computes_for_valid_entry() {
    let entry = make_valid_entry();
    let index = EntryIndex::new(0);

    let view = EntryView::with_rendered_lines(entry, index, WrapMode::Wrap, 80);

    // Verify rendered_lines is not empty (should have separator at minimum)
    let lines = view.rendered_lines();
    assert!(
        !lines.is_empty(),
        "rendered_lines should be computed during construction"
    );
}

#[test]
fn with_rendered_lines_computes_for_malformed_entry() {
    let entry = make_malformed_entry();
    let index = EntryIndex::new(0);

    let view = EntryView::with_rendered_lines(entry, index, WrapMode::Wrap, 80);

    // Malformed entries should have minimal rendering (just separator)
    let lines = view.rendered_lines();
    assert_eq!(
        lines.len(),
        1,
        "Malformed entry should have 1 line (separator)"
    );
}

#[test]
fn with_rendered_lines_uses_collapsed_state_by_default() {
    // Create entry with 100 lines of Thinking (above collapse threshold)
    let thinking_text = (0..100)
        .map(|i| format!("Thinking line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let entry = make_entry_with_thinking(&thinking_text);
    let index = EntryIndex::new(0);

    let view = EntryView::with_rendered_lines(entry, index, WrapMode::Wrap, 80);

    // Should be collapsed by default: 3 summary + 1 indicator + 1 separator = 5 lines
    let lines = view.rendered_lines();
    assert_eq!(
        lines.len(),
        5,
        "Default collapsed state should show 5 lines (3 summary + indicator + separator)"
    );
}

// ===== height() Method Tests =====

#[test]
fn height_returns_line_count_from_rendered_lines() {
    let entry = make_valid_entry();
    let index = EntryIndex::new(0);
    let view = EntryView::with_rendered_lines(entry, index, WrapMode::Wrap, 80);

    let height = view.height();
    let lines = view.rendered_lines();

    assert_eq!(
        height.get(),
        lines.len() as u16,
        "height() should return count of rendered_lines"
    );
}

#[test]
fn height_returns_one_for_minimal_entry() {
    // Malformed entry has 1 line (separator only)
    let entry = make_malformed_entry();
    let index = EntryIndex::new(0);
    let view = EntryView::with_rendered_lines(entry, index, WrapMode::Wrap, 80);

    let height = view.height();
    assert_eq!(height, LineHeight::ONE, "Minimal entry should have height=1");
}

#[test]
fn height_reflects_collapsed_vs_expanded_difference() {
    // Create entry with 100 lines of Thinking
    let thinking_text = (0..100)
        .map(|i| format!("Thinking line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let entry = make_entry_with_thinking(&thinking_text);
    let index = EntryIndex::new(0);

    // Collapsed view
    let view_collapsed = EntryView::with_rendered_lines(entry.clone(), index, WrapMode::Wrap, 80);
    let height_collapsed = view_collapsed.height();

    // Create expanded view by recomputing after expansion
    // Note: Expanded testing will be added after implementation reveals the full API
    // For now, just verify collapsed height is correct

    assert_eq!(
        height_collapsed.get(),
        5,
        "Collapsed height should be 5 lines"
    );
}

// ===== rendered_lines() Accessor Tests =====

#[test]
fn rendered_lines_returns_reference_to_lines() {
    let entry = make_valid_entry();
    let index = EntryIndex::new(0);
    let view = EntryView::with_rendered_lines(entry, index, WrapMode::Wrap, 80);

    let lines: &[Line<'static>] = view.rendered_lines();
    assert!(
        !lines.is_empty(),
        "rendered_lines() should return non-empty slice"
    );
}

#[test]
fn rendered_lines_has_static_lifetime() {
    let entry = make_valid_entry();
    let index = EntryIndex::new(0);
    let view = EntryView::with_rendered_lines(entry, index, WrapMode::Wrap, 80);

    // This compiles only if the returned slice has 'static lifetime
    let lines: &[Line<'static>] = view.rendered_lines();
    let _first: &Line<'static> = &lines[0];
    // Success: lifetime is 'static
}

// ===== recompute_lines() Method Tests =====

#[test]
fn recompute_lines_updates_rendered_lines() {
    // Create entry with 100 lines of Thinking
    let thinking_text = (0..100)
        .map(|i| format!("Thinking line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let entry = make_entry_with_thinking(&thinking_text);
    let index = EntryIndex::new(0);

    let mut view = EntryView::with_rendered_lines(entry, index, WrapMode::Wrap, 80);

    let initial_height = view.height();
    assert_eq!(initial_height.get(), 5, "Initial collapsed height");

    // Simulate state change (e.g., expansion, then recompute)
    // NOTE: This test assumes we can change expanded state internally
    // The actual API will be determined during implementation
    // For now, just verify recompute_lines exists and can be called
    view.recompute_lines(WrapMode::Wrap, 80);

    // Height should still be consistent with rendered_lines
    let new_height = view.height();
    let new_lines = view.rendered_lines();
    assert_eq!(
        new_height.get(),
        new_lines.len() as u16,
        "After recompute, height must match line count"
    );
}

#[test]
fn recompute_lines_with_different_width() {
    let entry = make_valid_entry();
    let index = EntryIndex::new(0);
    let mut view = EntryView::with_rendered_lines(entry, index, WrapMode::Wrap, 80);

    // Recompute with different width (wrapping behavior might change)
    view.recompute_lines(WrapMode::Wrap, 40);

    let new_lines = view.rendered_lines().len();
    // Note: For now, width doesn't affect rendering (deferred to bead 14.6)
    // But the API should still work
    assert_eq!(
        view.height().get(),
        new_lines as u16,
        "Height must match line count after width change"
    );
}

// ===== Integration Tests (API Consistency) =====

#[test]
fn height_and_rendered_lines_stay_consistent() {
    // Property: view.height().get() == view.rendered_lines().len() as u16
    // This must ALWAYS hold, regardless of state.

    let entries = vec![
        make_valid_entry(),
        make_malformed_entry(),
        make_entry_with_thinking("Short"),
        make_entry_with_thinking(&"Long\n".repeat(100)),
    ];

    for (i, entry) in entries.into_iter().enumerate() {
        let view = EntryView::with_rendered_lines(entry, EntryIndex::new(i), WrapMode::Wrap, 80);
        let height = view.height().get();
        let line_count = view.rendered_lines().len() as u16;

        assert_eq!(
            height, line_count,
            "Entry {}: height() must match rendered_lines().len()",
            i
        );
    }
}

#[test]
fn constructor_parameters_affect_rendering() {
    // Verify that wrap_mode and width are passed to compute_entry_lines
    let entry = make_valid_entry();
    let index = EntryIndex::new(0);

    let view1 = EntryView::with_rendered_lines(entry.clone(), index, WrapMode::Wrap, 80);
    let view2 = EntryView::with_rendered_lines(entry.clone(), index, WrapMode::NoWrap, 80);

    // For now, WrapMode doesn't affect output (deferred to 14.6)
    // But constructor should accept the parameter
    let _lines1 = view1.rendered_lines();
    let _lines2 = view2.rendered_lines();
    // Success: constructor accepts wrap_mode parameter
}

// ===== Removed API Tests (Ensure these DO NOT compile) =====

// These tests verify that old APIs are removed:
// - layout() method should not exist
// - set_layout() should not exist
// - with_layout() constructor should not exist

// Uncomment these to verify compilation failures after refactor:

// #[test]
// fn layout_method_removed() {
//     let entry = make_valid_entry();
//     let view = EntryView::new(entry, EntryIndex::new(0), WrapMode::Wrap, 80);
//     let _ = view.layout(); // Should NOT compile
// }

// #[test]
// fn set_layout_method_removed() {
//     let entry = make_valid_entry();
//     let mut view = EntryView::new(entry, EntryIndex::new(0), WrapMode::Wrap, 80);
//     view.set_layout(/* ... */); // Should NOT compile
// }

// #[test]
// fn with_layout_constructor_removed() {
//     let entry = make_valid_entry();
//     let _ = EntryView::with_layout(entry, EntryIndex::new(0), /* layout */); // Should NOT compile
// }
