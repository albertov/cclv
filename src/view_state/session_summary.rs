//! Session summary metadata for display in session list modal.

use crate::model::SessionId;
use crate::view_state::types::SessionIndex;
use chrono::{DateTime, Utc};

/// Summary metadata for a session, used in the session list modal.
///
/// This is a read-only snapshot of session state for display purposes.
/// Computed from SessionViewState on demand.
///
/// # FR-009: Display session metadata including:
/// - Session number (index + 1)
/// - Start timestamp
/// - Message count
#[derive(Debug, Clone)]
pub struct SessionSummary {
    /// Validated index of this session.
    index: SessionIndex,

    /// Session identifier (UUID).
    session_id: SessionId,

    /// Total message count in main conversation.
    message_count: usize,

    /// Timestamp of first entry in session (if available).
    start_time: Option<DateTime<Utc>>,

    /// Number of subagents spawned in this session.
    subagent_count: usize,
}

impl SessionSummary {
    /// Create a new session summary.
    ///
    /// # Arguments
    /// - `index`: Validated session index
    /// - `session_id`: Session UUID
    /// - `message_count`: Number of messages in main conversation
    /// - `start_time`: Timestamp of first entry
    /// - `subagent_count`: Number of subagents
    pub fn new(
        index: SessionIndex,
        session_id: SessionId,
        message_count: usize,
        start_time: Option<DateTime<Utc>>,
        subagent_count: usize,
    ) -> Self {
        Self {
            index,
            session_id,
            message_count,
            start_time,
            subagent_count,
        }
    }

    /// Session index.
    pub fn index(&self) -> SessionIndex {
        self.index
    }

    /// Session ID.
    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    /// Message count in main conversation.
    pub fn message_count(&self) -> usize {
        self.message_count
    }

    /// Start time of session.
    pub fn start_time(&self) -> Option<DateTime<Utc>> {
        self.start_time
    }

    /// Number of subagents.
    pub fn subagent_count(&self) -> usize {
        self.subagent_count
    }

    /// Format for display in session list.
    ///
    /// Returns: "Session N: X messages, Y subagents (HH:MM)"
    pub fn display_line(&self) -> String {
        let time_str = self
            .start_time
            .map(|t| t.format(" (%H:%M)").to_string())
            .unwrap_or_default();

        format!(
            "Session {}: {} messages, {} subagents{}",
            self.index.display(),
            self.message_count,
            self.subagent_count,
            time_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_session_id() -> SessionId {
        SessionId::new("550e8400-e29b-41d4-a716-446655440000").unwrap()
    }

    fn make_test_index() -> SessionIndex {
        SessionIndex::new(0, 3).unwrap()
    }

    fn make_test_time() -> DateTime<Utc> {
        chrono::NaiveDate::from_ymd_opt(2024, 1, 15)
            .unwrap()
            .and_hms_opt(14, 30, 45)
            .unwrap()
            .and_utc()
    }

    #[test]
    fn new_creates_session_summary() {
        let index = make_test_index();
        let session_id = make_test_session_id();
        let start_time = Some(make_test_time());

        let summary = SessionSummary::new(index, session_id.clone(), 10, start_time, 3);

        assert_eq!(summary.index(), index);
        assert_eq!(summary.session_id(), &session_id);
        assert_eq!(summary.message_count(), 10);
        assert_eq!(summary.start_time(), start_time);
        assert_eq!(summary.subagent_count(), 3);
    }

    #[test]
    fn new_handles_none_start_time() {
        let index = make_test_index();
        let session_id = make_test_session_id();

        let summary = SessionSummary::new(index, session_id, 5, None, 1);

        assert_eq!(summary.start_time(), None);
    }

    #[test]
    fn index_returns_session_index() {
        let index = SessionIndex::new(2, 5).unwrap();
        let session_id = make_test_session_id();

        let summary = SessionSummary::new(index, session_id, 7, None, 0);

        assert_eq!(summary.index(), index);
    }

    #[test]
    fn session_id_returns_reference() {
        let index = make_test_index();
        let session_id = make_test_session_id();

        let summary = SessionSummary::new(index, session_id.clone(), 1, None, 0);

        assert_eq!(summary.session_id(), &session_id);
    }

    #[test]
    fn message_count_returns_count() {
        let index = make_test_index();
        let session_id = make_test_session_id();

        let summary = SessionSummary::new(index, session_id, 42, None, 0);

        assert_eq!(summary.message_count(), 42);
    }

    #[test]
    fn start_time_returns_time() {
        let index = make_test_index();
        let session_id = make_test_session_id();
        let start_time = Some(make_test_time());

        let summary = SessionSummary::new(index, session_id, 1, start_time, 0);

        assert_eq!(summary.start_time(), start_time);
    }

    #[test]
    fn subagent_count_returns_count() {
        let index = make_test_index();
        let session_id = make_test_session_id();

        let summary = SessionSummary::new(index, session_id, 1, None, 7);

        assert_eq!(summary.subagent_count(), 7);
    }

    #[test]
    fn display_line_with_time() {
        let index = SessionIndex::new(0, 3).unwrap(); // Display as "Session 1"
        let session_id = make_test_session_id();
        let start_time = Some(make_test_time()); // 14:30

        let summary = SessionSummary::new(index, session_id, 10, start_time, 3);

        let result = summary.display_line();
        assert_eq!(result, "Session 1: 10 messages, 3 subagents (14:30)");
    }

    #[test]
    fn display_line_without_time() {
        let index = SessionIndex::new(1, 3).unwrap(); // Display as "Session 2"
        let session_id = make_test_session_id();

        let summary = SessionSummary::new(index, session_id, 5, None, 2);

        let result = summary.display_line();
        assert_eq!(result, "Session 2: 5 messages, 2 subagents");
    }

    #[test]
    fn display_line_zero_subagents() {
        let index = SessionIndex::new(2, 3).unwrap(); // Display as "Session 3"
        let session_id = make_test_session_id();

        let summary = SessionSummary::new(index, session_id, 1, None, 0);

        let result = summary.display_line();
        assert_eq!(result, "Session 3: 1 messages, 0 subagents");
    }

    #[test]
    fn display_line_large_numbers() {
        let index = SessionIndex::new(99, 100).unwrap(); // Display as "Session 100"
        let session_id = make_test_session_id();
        let start_time = Some(make_test_time());

        let summary = SessionSummary::new(index, session_id, 999, start_time, 42);

        let result = summary.display_line();
        assert_eq!(result, "Session 100: 999 messages, 42 subagents (14:30)");
    }
}
