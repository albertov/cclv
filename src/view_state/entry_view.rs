//! Entry view with per-entry state and precomputed rendered lines.

#[allow(unused_imports)] // Used after stub implementation
use super::renderer::compute_entry_lines;
use super::types::{EntryIndex, LineHeight};
use crate::model::ConversationEntry;
use crate::state::WrapMode;
use ratatui::text::Line;

/// A conversation entry with precomputed rendered lines and presentation state.
///
/// EntryView OWNS the domain entry (ConversationEntry) rather than
/// referencing it. This provides:
/// - Cache locality (entry + rendered lines + view state in same allocation)
/// - No lifetime complexity
/// - Simple streaming append
/// - O(1) access to per-entry view state (no HashSet lookups)
///
/// # Ownership (FR-002)
/// View-state layer owns domain data. Entries are parsed directly
/// into EntryView during JSON processing.
///
/// # Source of Truth for Height
/// The `rendered_lines` field is THE source of truth for entry height.
/// `height()` returns `LineHeight` based on `rendered_lines.len()`.
/// This ensures perfect consistency between computed layout and actual rendering.
///
/// # Per-Entry Presentation State
/// - `expanded`: Whether entry shows full content or collapsed summary (FR-031)
/// - `wrap_override`: Optional per-entry wrap mode override (FR-048)
///
/// # Malformed Entries
/// Malformed entries have minimal rendering (separator line only).
/// They still occupy a slot in the entry list to preserve index stability.
#[derive(Debug, Clone)]
pub struct EntryView {
    /// The domain entry (owned).
    entry: ConversationEntry,
    /// Index of this entry within its conversation.
    /// This is the canonical reference for entries.
    index: EntryIndex,
    /// Precomputed rendered lines (source of truth for height).
    /// These are cached ratatui Lines ready for rendering.
    #[allow(dead_code)] // Used after stub implementation
    rendered_lines: Vec<Line<'static>>,
    /// Whether this entry is expanded (shows full content).
    /// Collapsed entries show summary + "(+N more lines)" indicator.
    expanded: bool,
    /// Per-entry wrap mode override.
    /// `None` = use global wrap mode.
    /// `Some(mode)` = override global with this specific mode.
    wrap_override: Option<WrapMode>,
}

impl EntryView {
    /// Default collapse threshold (lines before collapsing).
    #[allow(dead_code)] // Used after stub implementation
    const COLLAPSE_THRESHOLD: usize = 10;

    /// Default summary lines (shown when collapsed).
    #[allow(dead_code)] // Used after stub implementation
    const SUMMARY_LINES: usize = 3;

    /// Create new EntryView with minimal state (for initial construction).
    ///
    /// This constructor creates an EntryView with empty rendered_lines.
    /// Call `recompute_lines()` after construction to populate rendered_lines.
    ///
    /// This is used during ConversationViewState construction where layout
    /// parameters (width, wrap_mode) are not yet available.
    ///
    /// # Arguments
    /// * `entry` - Domain entry to wrap
    /// * `index` - Position within conversation
    pub fn new(entry: ConversationEntry, index: EntryIndex) -> Self {
        Self {
            entry,
            index,
            rendered_lines: vec![Line::from("")], // Minimal placeholder (1 line minimum)
            expanded: false,
            wrap_override: None,
        }
    }

    /// Create new EntryView with precomputed rendered lines.
    ///
    /// This constructor:
    /// - Calls compute_entry_lines to generate rendered output
    /// - Starts in collapsed state (expanded=false)
    /// - Uses default collapse thresholds (10/3)
    /// - Has no wrap override (uses global wrap mode)
    ///
    /// # Arguments
    /// * `entry` - Domain entry to wrap
    /// * `index` - Position within conversation
    /// * `wrap_mode` - Effective wrap mode for this entry
    /// * `width` - Viewport width for text wrapping
    pub fn with_rendered_lines(
        _entry: ConversationEntry,
        _index: EntryIndex,
        _wrap_mode: WrapMode,
        _width: u16,
    ) -> Self {
        todo!("EntryView::with_rendered_lines - compute rendered_lines via compute_entry_lines")
    }

    /// Get the entry index (0-based).
    pub fn index(&self) -> EntryIndex {
        self.index
    }

    /// Get the display index (1-based for UI).
    pub fn display_index(&self) -> usize {
        self.index.display()
    }

    /// Get reference to the domain entry.
    pub fn entry(&self) -> &ConversationEntry {
        &self.entry
    }

    /// Get the entry UUID (if valid entry).
    pub fn uuid(&self) -> Option<&crate::model::EntryUuid> {
        match &self.entry {
            ConversationEntry::Valid(log_entry) => Some(log_entry.uuid()),
            ConversationEntry::Malformed(_) => None,
        }
    }

    /// Get the height of this entry (count of rendered lines).
    ///
    /// This is derived from `rendered_lines.len()` and is the source of truth
    /// for entry height. The returned LineHeight is guaranteed to be >= 1
    /// for all entries (minimum is separator line).
    pub fn height(&self) -> LineHeight {
        todo!("EntryView::height - return LineHeight from rendered_lines.len()")
    }

    /// Get reference to the rendered lines.
    ///
    /// These are precomputed ratatui Lines ready for rendering.
    /// The slice has 'static lifetime because all content is owned.
    pub fn rendered_lines(&self) -> &[Line<'static>] {
        todo!("EntryView::rendered_lines - return slice of rendered_lines")
    }

    /// Recompute rendered lines after state change.
    ///
    /// This is called by ConversationViewState when:
    /// - Viewport width changes
    /// - Wrap mode changes
    /// - Entry expand/collapse state changes
    ///
    /// This is `pub(crate)` because only ConversationViewState should
    /// trigger recomputation (to maintain HeightIndex consistency).
    pub(crate) fn recompute_lines(&mut self, _wrap_mode: WrapMode, _width: u16) {
        todo!("EntryView::recompute_lines - call compute_entry_lines and update rendered_lines")
    }

    /// Check if this entry is expanded.
    pub fn is_expanded(&self) -> bool {
        self.expanded
    }

    /// Get the wrap mode override.
    pub fn wrap_override(&self) -> Option<WrapMode> {
        self.wrap_override
    }

    /// Get the effective wrap mode (override or global fallback).
    pub fn effective_wrap(&self, global: WrapMode) -> WrapMode {
        self.wrap_override.unwrap_or(global)
    }

    // NOTE: Mutation methods are pub(crate) for now to allow ConversationViewState
    // to call them during the refactoring. After the refactoring is complete,
    // ConversationViewState will handle recompute_lines() and these can be private.

    /// Set the expanded state (internal - called by ConversationViewState).
    pub(crate) fn set_expanded(&mut self, expanded: bool) {
        self.expanded = expanded;
    }

    /// Toggle expanded state and return the new state (internal - called by ConversationViewState).
    pub(crate) fn toggle_expanded(&mut self) -> bool {
        self.expanded = !self.expanded;
        self.expanded
    }

    /// Set the wrap mode override (internal - called by ConversationViewState).
    pub(crate) fn set_wrap_override(&mut self, mode: Option<WrapMode>) {
        self.wrap_override = mode;
    }

    // TEMPORARY COMPATIBILITY SHIMS (will be removed after refactoring)
    // These allow the old API to continue working during migration.

    /// Temporary compatibility shim for layout access.
    /// Returns a placeholder EntryLayout based on rendered_lines height.
    #[allow(dead_code)]
    pub(crate) fn layout(&self) -> super::layout::EntryLayout {
        use super::layout::EntryLayout;
        use super::types::LineOffset;
        // Return placeholder layout with height from rendered_lines
        // cumulative_y is meaningless here (will be fixed in proper migration)
        EntryLayout::new(self.height(), LineOffset::new(0))
    }

    /// Temporary compatibility shim for set_layout.
    /// Does nothing - layout is now derived from rendered_lines.
    #[allow(dead_code)]
    pub(crate) fn set_layout(&mut self, _layout: super::layout::EntryLayout) {
        // No-op: layout is now computed from rendered_lines
        // This shim allows old code to compile during migration
    }
}

// Include refactor tests
#[cfg(test)]
#[path = "entry_view_refactor_tests.rs"]
mod refactor_tests;

// Keep existing tests for now (will update after implementation)
#[cfg(test)]
mod legacy_tests {
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

    #[allow(dead_code)] // Will be used when updating more tests
    fn make_malformed_entry() -> ConversationEntry {
        ConversationEntry::Malformed(MalformedEntry::new(
            42,
            "bad json",
            "Parse error",
            Some(make_session_id("session-1")),
        ))
    }

    // ===== Legacy Tests (will be updated) =====
    // These tests use the OLD API and will fail with stubs.
    // We'll update them after implementing the new API.

    #[test]
    fn new_creates_entry_with_minimal_state() {
        let entry = make_valid_entry();
        let index = EntryIndex::new(0);

        // NEW API: EntryView::new creates minimal placeholder
        let view = EntryView::new(entry, index);

        assert_eq!(view.index(), index);
        assert!(
            !view.is_expanded(),
            "Default state should be collapsed (not expanded)"
        );
        assert_eq!(
            view.wrap_override(),
            None,
            "Default should have no wrap override"
        );
        // rendered_lines will be minimal placeholder (1 line)
        assert_eq!(view.rendered_lines().len(), 1, "Should have placeholder line");
    }

    // Additional legacy tests omitted for brevity.
    // They will be updated in the implementation phase.
}
