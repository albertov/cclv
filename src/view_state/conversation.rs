//! View-state for a single conversation

#![allow(dead_code)] // Will be used by tests and other modules

use super::{
    entry_view::EntryView,
    hit_test::HitTestResult,
    layout_params::LayoutParams,
    scroll::ScrollPosition,
    types::{EntryIndex, LineHeight, LineOffset, ViewportDimensions},
    visible_range::VisibleRange,
};
use crate::model::ConversationEntry;
use crate::state::app_state::WrapMode;

/// View-state for a single conversation.
///
/// Contains:
/// - Owned entries with computed layouts
/// - Current scroll position
/// - Cached total height
/// - Layout validity tracking
///
/// # Layout Computation (FR-020 to FR-024)
/// Layout is computed lazily on first render or explicitly.
/// Heights depend on viewport width, expand state, and wrap mode.
/// Cumulative Y offsets are maintained as running sum.
///
/// # Visible Range (FR-030, FR-031)
/// `visible_range()` uses binary search on cumulative_y for O(log n) lookup.
///
/// # Hit Testing (FR-040 to FR-043)
/// `hit_test()` uses binary search on cumulative_y for O(log n) lookup.
#[derive(Debug, Clone)]
pub struct ConversationViewState {
    /// Entries with computed layouts and per-entry view state.
    entries: Vec<EntryView>,
    /// Current scroll position.
    scroll: ScrollPosition,
    /// Cached total height in lines (sum of all entry heights).
    total_height: usize,
    /// Index of currently focused entry (for keyboard navigation).
    /// `None` means no specific entry is focused.
    focused_message: Option<EntryIndex>,
    /// Global parameters used for last layout computation.
    last_layout_params: Option<LayoutParams>,
}

impl ConversationViewState {
    /// Create new conversation view-state from entries.
    /// Layout is not computed until `recompute_layout` is called.
    pub fn new(_entries: Vec<ConversationEntry>) -> Self {
        todo!("ConversationViewState::new")
    }

    /// Create empty conversation view-state.
    pub fn empty() -> Self {
        todo!("ConversationViewState::empty")
    }

    // === Focus Management ===

    /// Get focused entry index.
    pub fn focused_message(&self) -> Option<EntryIndex> {
        todo!("ConversationViewState::focused_message")
    }

    /// Set focused entry index.
    /// Clamps to valid range if index >= len.
    pub fn set_focused_message(&mut self, _index: Option<EntryIndex>) {
        todo!("ConversationViewState::set_focused_message")
    }

    /// Get focused entry view, if any.
    pub fn focused_entry(&self) -> Option<&EntryView> {
        todo!("ConversationViewState::focused_entry")
    }

    /// Get mutable focused entry view, if any.
    pub fn focused_entry_mut(&mut self) -> Option<&mut EntryView> {
        todo!("ConversationViewState::focused_entry_mut")
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        todo!("ConversationViewState::len")
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        todo!("ConversationViewState::is_empty")
    }

    /// Get entry at index.
    pub fn get(&self, _index: EntryIndex) -> Option<&EntryView> {
        todo!("ConversationViewState::get")
    }

    /// Iterate over all entries.
    pub fn iter(&self) -> impl Iterator<Item = &EntryView> {
        self.entries.iter()
    }

    /// Current scroll position.
    pub fn scroll(&self) -> &ScrollPosition {
        todo!("ConversationViewState::scroll")
    }

    /// Set scroll position.
    pub fn set_scroll(&mut self, _position: ScrollPosition) {
        todo!("ConversationViewState::set_scroll")
    }

    /// Total height in lines.
    pub fn total_height(&self) -> usize {
        todo!("ConversationViewState::total_height")
    }

    /// Check if global layout params changed (width, global_wrap).
    /// Note: Per-entry state changes (expand, wrap_override) require
    /// targeted relayout via `relayout_entry` or `relayout_from`.
    pub fn needs_relayout(&self, _params: &LayoutParams) -> bool {
        todo!("ConversationViewState::needs_relayout")
    }

    /// Append new entries (streaming mode).
    /// New entries have default layout; call `recompute_layout` to update.
    pub fn append(&mut self, _entries: Vec<ConversationEntry>) {
        todo!("ConversationViewState::append")
    }

    /// Recompute layout for all entries.
    ///
    /// # Arguments
    /// - `params`: Current global layout parameters
    /// - `height_calculator`: Function to compute height for an entry
    ///   Receives: entry, expanded state, effective wrap mode
    pub fn recompute_layout<F>(&mut self, _params: LayoutParams, _height_calculator: F)
    where
        F: Fn(&ConversationEntry, bool, WrapMode) -> LineHeight,
    {
        todo!("ConversationViewState::recompute_layout")
    }

    /// Relayout from a specific entry index onward.
    /// Used after toggling expand/wrap on a single entry.
    /// More efficient than full relayout for single-entry changes.
    ///
    /// # Arguments
    /// - `from_index`: Index of first entry to relayout
    /// - `params`: Current global layout parameters
    /// - `height_calculator`: Function to compute height
    pub fn relayout_from<F>(&mut self, _from_index: EntryIndex, _params: LayoutParams, _height_calculator: F)
    where
        F: Fn(&ConversationEntry, bool, WrapMode) -> LineHeight,
    {
        todo!("ConversationViewState::relayout_from")
    }

    /// Toggle expand state for entry at index and relayout.
    /// Returns new expanded state, or None if index out of bounds.
    pub fn toggle_expand<F>(&mut self, _index: EntryIndex, _params: LayoutParams, _height_calculator: F) -> Option<bool>
    where
        F: Fn(&ConversationEntry, bool, WrapMode) -> LineHeight,
    {
        todo!("ConversationViewState::toggle_expand")
    }

    /// Set wrap override for entry at index and relayout.
    /// Returns previous wrap override, or None if index out of bounds.
    pub fn set_wrap_override<F>(
        &mut self,
        _index: EntryIndex,
        _wrap: Option<WrapMode>,
        _params: LayoutParams,
        _height_calculator: F,
    ) -> Option<Option<WrapMode>>
    where
        F: Fn(&ConversationEntry, bool, WrapMode) -> LineHeight,
    {
        todo!("ConversationViewState::set_wrap_override")
    }

    /// Compute visible range using binary search.
    /// O(log n) complexity.
    ///
    /// # Arguments
    /// - `viewport`: Viewport dimensions
    ///
    /// # Returns
    /// Range of entry indices that are visible.
    pub fn visible_range(&self, _viewport: ViewportDimensions) -> VisibleRange {
        todo!("ConversationViewState::visible_range")
    }

    /// Hit-test a screen coordinate.
    /// O(log n) complexity.
    ///
    /// # Arguments
    /// - `screen_y`: Y coordinate relative to viewport top
    /// - `screen_x`: X coordinate
    /// - `scroll_offset`: Current scroll offset
    ///
    /// # Returns
    /// What entry (if any) was hit.
    pub fn hit_test(&self, _screen_y: u16, _screen_x: u16, _scroll_offset: LineOffset) -> HitTestResult {
        todo!("ConversationViewState::hit_test")
    }

    /// Get cumulative_y for entry at index (for scroll resolution).
    pub fn entry_cumulative_y(&self, _index: EntryIndex) -> Option<LineOffset> {
        todo!("ConversationViewState::entry_cumulative_y")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        EntryMetadata, EntryType, EntryUuid, LogEntry, MalformedEntry, Message, MessageContent,
        Role, SessionId,
    };

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

    fn make_valid_entry(uuid: &str) -> ConversationEntry {
        let log_entry = LogEntry::new(
            make_entry_uuid(uuid),
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

    // Mock height calculator: returns fixed height for testing
    fn fixed_height_calculator(height: u16) -> impl Fn(&ConversationEntry, bool, WrapMode) -> LineHeight {
        move |_entry, _expanded, _wrap| LineHeight::new(height).unwrap()
    }

    // === ConversationViewState::new Tests ===

    #[test]
    fn new_creates_view_state_from_entries() {
        let entries = vec![
            make_valid_entry("uuid-1"),
            make_valid_entry("uuid-2"),
            make_valid_entry("uuid-3"),
        ];

        let state = ConversationViewState::new(entries);

        assert_eq!(state.len(), 3, "Should have 3 entries");
        assert!(!state.is_empty(), "Should not be empty");
    }

    #[test]
    fn new_starts_with_no_layout() {
        let entries = vec![make_valid_entry("uuid-1")];
        let state = ConversationViewState::new(entries);

        assert_eq!(state.total_height(), 0, "Total height should be 0 before layout");
        assert!(
            state.last_layout_params.is_none(),
            "Should have no layout params until first layout"
        );
    }

    #[test]
    fn new_starts_scrolled_to_top() {
        let entries = vec![make_valid_entry("uuid-1")];
        let state = ConversationViewState::new(entries);

        assert_eq!(state.scroll(), &ScrollPosition::Top, "Should start at top");
    }

    #[test]
    fn new_starts_with_no_focused_message() {
        let entries = vec![make_valid_entry("uuid-1")];
        let state = ConversationViewState::new(entries);

        assert_eq!(
            state.focused_message(),
            None,
            "Should have no focused message initially"
        );
    }

    #[test]
    fn new_assigns_correct_indices() {
        let entries = vec![
            make_valid_entry("uuid-1"),
            make_valid_entry("uuid-2"),
            make_valid_entry("uuid-3"),
        ];
        let state = ConversationViewState::new(entries);

        assert_eq!(state.get(EntryIndex::new(0)).unwrap().index(), EntryIndex::new(0));
        assert_eq!(state.get(EntryIndex::new(1)).unwrap().index(), EntryIndex::new(1));
        assert_eq!(state.get(EntryIndex::new(2)).unwrap().index(), EntryIndex::new(2));
    }

    // === ConversationViewState::empty Tests ===

    #[test]
    fn empty_creates_empty_state() {
        let state = ConversationViewState::empty();

        assert_eq!(state.len(), 0);
        assert!(state.is_empty());
    }

    // === recompute_layout Tests ===

    #[test]
    fn recompute_layout_sets_total_height() {
        let entries = vec![
            make_valid_entry("uuid-1"),
            make_valid_entry("uuid-2"),
            make_valid_entry("uuid-3"),
        ];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(5));

        assert_eq!(
            state.total_height(),
            15,
            "Total height should be 3 entries * 5 lines each"
        );
    }

    #[test]
    fn recompute_layout_maintains_cumulative_y_invariant() {
        let entries = vec![
            make_valid_entry("uuid-1"),
            make_valid_entry("uuid-2"),
            make_valid_entry("uuid-3"),
        ];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(5));

        // Verify cumulative_y invariant: cumulative_y[i] = sum(height[0..i])
        assert_eq!(state.get(EntryIndex::new(0)).unwrap().layout().cumulative_y().get(), 0);
        assert_eq!(state.get(EntryIndex::new(1)).unwrap().layout().cumulative_y().get(), 5);
        assert_eq!(state.get(EntryIndex::new(2)).unwrap().layout().cumulative_y().get(), 10);
    }

    #[test]
    fn recompute_layout_stores_params() {
        let entries = vec![make_valid_entry("uuid-1")];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(5));

        assert_eq!(state.last_layout_params, Some(params));
    }

    // === visible_range Tests (Binary Search) ===

    #[test]
    fn visible_range_empty_state_returns_empty_range() {
        let state = ConversationViewState::empty();
        let viewport = ViewportDimensions::new(80, 24);

        let range = state.visible_range(viewport);

        assert!(range.is_empty());
        assert_eq!(range.start_index, EntryIndex::new(0));
        assert_eq!(range.end_index, EntryIndex::new(0));
    }

    #[test]
    fn visible_range_from_top_shows_first_entries() {
        let entries = vec![
            make_valid_entry("uuid-1"),
            make_valid_entry("uuid-2"),
            make_valid_entry("uuid-3"),
            make_valid_entry("uuid-4"),
        ];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(10)); // Each entry is 10 lines
        state.set_scroll(ScrollPosition::Top);

        let viewport = ViewportDimensions::new(80, 24); // 24 line viewport

        let range = state.visible_range(viewport);

        // Viewport shows lines 0-23, entries are at y=[0, 10, 20, 30]
        // Entry 0: y=0..10 (visible)
        // Entry 1: y=10..20 (visible)
        // Entry 2: y=20..30 (partially visible, starts at line 20)
        // Entry 3: y=30..40 (not visible)
        assert_eq!(range.start_index, EntryIndex::new(0));
        assert_eq!(range.end_index, EntryIndex::new(3)); // Exclusive, so 0,1,2 are visible
    }

    #[test]
    fn visible_range_scrolled_shows_middle_entries() {
        let entries = vec![
            make_valid_entry("uuid-1"),
            make_valid_entry("uuid-2"),
            make_valid_entry("uuid-3"),
            make_valid_entry("uuid-4"),
            make_valid_entry("uuid-5"),
        ];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(10)); // Each entry is 10 lines
        state.set_scroll(ScrollPosition::AtLine(LineOffset::new(15)));

        let viewport = ViewportDimensions::new(80, 24); // Viewport shows lines 15-38

        let range = state.visible_range(viewport);

        // Entry 0: y=0..10 (not visible, ends before viewport)
        // Entry 1: y=10..20 (partially visible, overlaps viewport start)
        // Entry 2: y=20..30 (visible)
        // Entry 3: y=30..40 (partially visible, starts in viewport)
        // Entry 4: y=40..50 (not visible)
        assert_eq!(range.start_index, EntryIndex::new(1));
        assert_eq!(range.end_index, EntryIndex::new(4)); // 1,2,3 visible
    }

    #[test]
    fn visible_range_at_bottom_shows_last_entries() {
        let entries = vec![
            make_valid_entry("uuid-1"),
            make_valid_entry("uuid-2"),
            make_valid_entry("uuid-3"),
            make_valid_entry("uuid-4"),
        ];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(10)); // Total height = 40
        state.set_scroll(ScrollPosition::Bottom);

        let viewport = ViewportDimensions::new(80, 24); // Viewport shows lines 16-39 (40-24=16)

        let range = state.visible_range(viewport);

        // Entry 0: y=0..10 (not visible)
        // Entry 1: y=10..20 (partially visible, ends at line 20)
        // Entry 2: y=20..30 (visible)
        // Entry 3: y=30..40 (visible)
        assert_eq!(range.start_index, EntryIndex::new(1));
        assert_eq!(range.end_index, EntryIndex::new(4)); // 1,2,3 visible
    }

    // === relayout_from Tests ===

    #[test]
    fn relayout_from_updates_from_index_onward() {
        let entries = vec![
            make_valid_entry("uuid-1"),
            make_valid_entry("uuid-2"),
            make_valid_entry("uuid-3"),
        ];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(10));

        // Initial: [0, 10, 20], total=30
        // Change entry 1 to height 20
        state.entries[1].set_expanded(true); // Simulate expansion

        let variable_height = |_entry: &ConversationEntry, expanded: bool, _wrap: WrapMode| {
            if expanded {
                LineHeight::new(20).unwrap()
            } else {
                LineHeight::new(10).unwrap()
            }
        };

        state.relayout_from(EntryIndex::new(1), params, variable_height);

        // After relayout from 1: [0, 10, 30], total=40
        assert_eq!(state.get(EntryIndex::new(0)).unwrap().layout().cumulative_y().get(), 0);
        assert_eq!(state.get(EntryIndex::new(1)).unwrap().layout().cumulative_y().get(), 10);
        assert_eq!(state.get(EntryIndex::new(2)).unwrap().layout().cumulative_y().get(), 30); // 10 + 20
        assert_eq!(state.total_height(), 40);
    }

    #[test]
    fn relayout_from_zero_is_equivalent_to_full_relayout() {
        let entries = vec![
            make_valid_entry("uuid-1"),
            make_valid_entry("uuid-2"),
            make_valid_entry("uuid-3"),
        ];

        let mut state1 = ConversationViewState::new(entries.clone());
        let mut state2 = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);

        state1.recompute_layout(params, fixed_height_calculator(10));
        state2.relayout_from(EntryIndex::new(0), params, fixed_height_calculator(10));

        // Both should produce identical layout
        assert_eq!(state1.total_height(), state2.total_height());
        for i in 0..3 {
            let idx = EntryIndex::new(i);
            assert_eq!(
                state1.get(idx).unwrap().layout().cumulative_y(),
                state2.get(idx).unwrap().layout().cumulative_y()
            );
        }
    }

    // === toggle_expand Tests ===

    #[test]
    fn toggle_expand_returns_new_state() {
        let entries = vec![make_valid_entry("uuid-1")];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(10));

        let result = state.toggle_expand(EntryIndex::new(0), params, fixed_height_calculator(10));

        assert_eq!(result, Some(true), "Should toggle to expanded");
        assert!(state.get(EntryIndex::new(0)).unwrap().is_expanded());
    }

    #[test]
    fn toggle_expand_returns_none_for_invalid_index() {
        let mut state = ConversationViewState::empty();
        let params = LayoutParams::new(80, WrapMode::Wrap);

        let result = state.toggle_expand(EntryIndex::new(0), params, fixed_height_calculator(10));

        assert_eq!(result, None);
    }

    #[test]
    fn toggle_expand_triggers_relayout() {
        let entries = vec![
            make_valid_entry("uuid-1"),
            make_valid_entry("uuid-2"),
        ];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        let variable_height = |_entry: &ConversationEntry, expanded: bool, _wrap: WrapMode| {
            if expanded {
                LineHeight::new(20).unwrap()
            } else {
                LineHeight::new(10).unwrap()
            }
        };

        state.recompute_layout(params, variable_height);
        // Initial: [0, 10], total=20

        state.toggle_expand(EntryIndex::new(0), params, variable_height);
        // After expanding entry 0: [0, 20], total=30

        assert_eq!(state.get(EntryIndex::new(0)).unwrap().layout().cumulative_y().get(), 0);
        assert_eq!(state.get(EntryIndex::new(1)).unwrap().layout().cumulative_y().get(), 20);
        assert_eq!(state.total_height(), 30);
    }

    // === hit_test Tests (Binary Search) ===

    #[test]
    fn hit_test_empty_state_returns_miss() {
        let state = ConversationViewState::empty();

        let result = state.hit_test(10, 10, LineOffset::new(0));

        assert_eq!(result, HitTestResult::Miss);
    }

    #[test]
    fn hit_test_finds_first_entry() {
        let entries = vec![
            make_valid_entry("uuid-1"),
            make_valid_entry("uuid-2"),
        ];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(10));

        // Click at screen_y=5 (no scroll) should hit entry 0 at line 5
        let result = state.hit_test(5, 10, LineOffset::new(0));

        assert_eq!(
            result,
            HitTestResult::Hit {
                entry_index: EntryIndex::new(0),
                line_in_entry: 5,
                column: 10
            }
        );
    }

    #[test]
    fn hit_test_finds_second_entry_with_scroll() {
        let entries = vec![
            make_valid_entry("uuid-1"),
            make_valid_entry("uuid-2"),
            make_valid_entry("uuid-3"),
        ];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(10)); // [0, 10, 20]

        // Click at screen_y=5 with scroll_offset=10
        // Absolute y = 10 + 5 = 15, which is in entry 1 (y=10..20)
        let result = state.hit_test(5, 20, LineOffset::new(10));

        assert_eq!(
            result,
            HitTestResult::Hit {
                entry_index: EntryIndex::new(1),
                line_in_entry: 5,
                column: 20
            }
        );
    }

    #[test]
    fn hit_test_beyond_content_returns_miss() {
        let entries = vec![make_valid_entry("uuid-1")];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(10)); // Total height = 10

        // Click at absolute y=15 (beyond entry 0 which ends at 10)
        let result = state.hit_test(15, 0, LineOffset::new(0));

        assert_eq!(result, HitTestResult::Miss);
    }

    // === needs_relayout Tests ===

    #[test]
    fn needs_relayout_true_when_params_change() {
        let entries = vec![make_valid_entry("uuid-1")];
        let mut state = ConversationViewState::new(entries);

        let params1 = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params1, fixed_height_calculator(10));

        let params2 = LayoutParams::new(120, WrapMode::Wrap); // Different width
        assert!(state.needs_relayout(&params2));
    }

    #[test]
    fn needs_relayout_false_when_params_unchanged() {
        let entries = vec![make_valid_entry("uuid-1")];
        let mut state = ConversationViewState::new(entries);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(10));

        assert!(!state.needs_relayout(&params));
    }

    // === append Tests ===

    #[test]
    fn append_adds_entries_to_end() {
        let mut state = ConversationViewState::new(vec![make_valid_entry("uuid-1")]);

        state.append(vec![make_valid_entry("uuid-2"), make_valid_entry("uuid-3")]);

        assert_eq!(state.len(), 3);
        assert_eq!(state.get(EntryIndex::new(2)).unwrap().index(), EntryIndex::new(2));
    }

    #[test]
    fn append_invalidates_layout() {
        let mut state = ConversationViewState::new(vec![make_valid_entry("uuid-1")]);

        let params = LayoutParams::new(80, WrapMode::Wrap);
        state.recompute_layout(params, fixed_height_calculator(10));

        state.append(vec![make_valid_entry("uuid-2")]);

        assert!(
            state.last_layout_params.is_none(),
            "Appending should invalidate layout params"
        );
    }
}
