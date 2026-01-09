//! Which session is currently being viewed.

use crate::view_state::types::SessionIndex;

/// Which session is currently being viewed.
///
/// # States
/// - `Latest`: Follow the most recent session (enables live tailing)
/// - `Pinned(index)`: View a specific historical session (disables live tailing)
///
/// # Cardinality
/// - Latest: 1 state
/// - Pinned: N states (where N = session count)
/// - Total: N + 1 states (all valid)
/// - Precision: 1.0
///
/// # Invariant
/// `Pinned(idx)` always holds a valid SessionIndex (validated at construction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewedSession {
    /// Follow the latest (last) session. Enables live tailing.
    #[default]
    Latest,

    /// Pinned to a specific session. Disables live tailing.
    Pinned(SessionIndex),
}

impl ViewedSession {
    /// Create a pinned view to specific session.
    ///
    /// Returns `None` if index is invalid for current session count.
    pub fn pinned(_index: usize, _session_count: usize) -> Option<Self> {
        todo!("ViewedSession::pinned")
    }

    /// Check if viewing the last session.
    ///
    /// Used to determine if live tailing should be enabled.
    pub fn is_last(&self, _session_count: usize) -> bool {
        todo!("ViewedSession::is_last")
    }

    /// Get the effective session index.
    ///
    /// For `Latest`, returns the last session index.
    /// For `Pinned`, returns the pinned index.
    pub fn effective_index(&self, _session_count: usize) -> Option<SessionIndex> {
        todo!("ViewedSession::effective_index")
    }

    /// Move to next session (toward latest).
    ///
    /// If at last session, switches to `Latest` mode.
    pub fn next(&self, _session_count: usize) -> Self {
        todo!("ViewedSession::next")
    }

    /// Move to previous session (toward first).
    ///
    /// If at first session, stays at first.
    pub fn prev(&self, _session_count: usize) -> Self {
        todo!("ViewedSession::prev")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod default {
        use super::*;

        #[test]
        fn default_is_latest() {
            assert_eq!(ViewedSession::default(), ViewedSession::Latest);
        }
    }

    mod pinned {
        use super::*;

        #[test]
        fn pinned_accepts_valid_index() {
            let result = ViewedSession::pinned(0, 3);
            assert!(result.is_some());
        }

        #[test]
        fn pinned_accepts_middle_index() {
            let result = ViewedSession::pinned(1, 3);
            assert!(result.is_some());
        }

        #[test]
        fn pinned_accepts_last_index() {
            let result = ViewedSession::pinned(2, 3);
            assert!(result.is_some());
        }

        #[test]
        fn pinned_rejects_out_of_bounds() {
            let result = ViewedSession::pinned(3, 3);
            assert!(result.is_none());
        }

        #[test]
        fn pinned_rejects_far_out_of_bounds() {
            let result = ViewedSession::pinned(100, 3);
            assert!(result.is_none());
        }

        #[test]
        fn pinned_creates_pinned_variant() {
            let result = ViewedSession::pinned(1, 3).unwrap();
            match result {
                ViewedSession::Pinned(idx) => assert_eq!(idx.get(), 1),
                ViewedSession::Latest => panic!("Expected Pinned variant"),
            }
        }
    }

    mod is_last {
        use super::*;

        #[test]
        fn latest_is_always_last() {
            assert!(ViewedSession::Latest.is_last(1));
            assert!(ViewedSession::Latest.is_last(3));
            assert!(ViewedSession::Latest.is_last(100));
        }

        #[test]
        fn pinned_to_last_session_is_last() {
            let session = ViewedSession::pinned(2, 3).unwrap();
            assert!(session.is_last(3));
        }

        #[test]
        fn pinned_to_first_session_not_last() {
            let session = ViewedSession::pinned(0, 3).unwrap();
            assert!(!session.is_last(3));
        }

        #[test]
        fn pinned_to_middle_session_not_last() {
            let session = ViewedSession::pinned(1, 3).unwrap();
            assert!(!session.is_last(3));
        }
    }

    mod effective_index {
        use super::*;

        #[test]
        fn latest_returns_last_session_index() {
            let result = ViewedSession::Latest.effective_index(3);
            assert!(result.is_some());
            assert_eq!(result.unwrap().get(), 2);
        }

        #[test]
        fn latest_with_single_session() {
            let result = ViewedSession::Latest.effective_index(1);
            assert!(result.is_some());
            assert_eq!(result.unwrap().get(), 0);
        }

        #[test]
        fn latest_with_no_sessions_returns_none() {
            let result = ViewedSession::Latest.effective_index(0);
            assert!(result.is_none());
        }

        #[test]
        fn pinned_returns_pinned_index() {
            let session = ViewedSession::pinned(1, 3).unwrap();
            let result = session.effective_index(3);
            assert!(result.is_some());
            assert_eq!(result.unwrap().get(), 1);
        }

        #[test]
        fn pinned_to_first_returns_first_index() {
            let session = ViewedSession::pinned(0, 3).unwrap();
            let result = session.effective_index(3);
            assert!(result.is_some());
            assert_eq!(result.unwrap().get(), 0);
        }

        #[test]
        fn pinned_to_last_returns_last_index() {
            let session = ViewedSession::pinned(2, 3).unwrap();
            let result = session.effective_index(3);
            assert!(result.is_some());
            assert_eq!(result.unwrap().get(), 2);
        }
    }

    mod next {
        use super::*;

        #[test]
        fn latest_next_stays_latest() {
            let result = ViewedSession::Latest.next(3);
            assert_eq!(result, ViewedSession::Latest);
        }

        #[test]
        fn pinned_first_moves_to_second() {
            let session = ViewedSession::pinned(0, 3).unwrap();
            let result = session.next(3);
            match result {
                ViewedSession::Pinned(idx) => assert_eq!(idx.get(), 1),
                ViewedSession::Latest => panic!("Expected Pinned variant"),
            }
        }

        #[test]
        fn pinned_middle_moves_forward() {
            let session = ViewedSession::pinned(1, 3).unwrap();
            let result = session.next(3);
            match result {
                ViewedSession::Pinned(idx) => assert_eq!(idx.get(), 2),
                ViewedSession::Latest => panic!("Expected Pinned variant"),
            }
        }

        #[test]
        fn pinned_last_switches_to_latest() {
            let session = ViewedSession::pinned(2, 3).unwrap();
            let result = session.next(3);
            assert_eq!(result, ViewedSession::Latest);
        }
    }

    mod prev {
        use super::*;

        #[test]
        fn latest_with_multiple_sessions_pins_to_second_last() {
            let result = ViewedSession::Latest.prev(3);
            match result {
                ViewedSession::Pinned(idx) => assert_eq!(idx.get(), 1),
                ViewedSession::Latest => panic!("Expected Pinned variant"),
            }
        }

        #[test]
        fn latest_with_single_session_stays_latest() {
            let result = ViewedSession::Latest.prev(1);
            assert_eq!(result, ViewedSession::Latest);
        }

        #[test]
        fn latest_with_no_sessions_stays_latest() {
            let result = ViewedSession::Latest.prev(0);
            assert_eq!(result, ViewedSession::Latest);
        }

        #[test]
        fn pinned_second_moves_to_first() {
            let session = ViewedSession::pinned(1, 3).unwrap();
            let result = session.prev(3);
            match result {
                ViewedSession::Pinned(idx) => assert_eq!(idx.get(), 0),
                ViewedSession::Latest => panic!("Expected Pinned variant"),
            }
        }

        #[test]
        fn pinned_last_moves_to_second_last() {
            let session = ViewedSession::pinned(2, 3).unwrap();
            let result = session.prev(3);
            match result {
                ViewedSession::Pinned(idx) => assert_eq!(idx.get(), 1),
                ViewedSession::Latest => panic!("Expected Pinned variant"),
            }
        }

        #[test]
        fn pinned_first_stays_at_first() {
            let session = ViewedSession::pinned(0, 3).unwrap();
            let result = session.prev(3);
            match result {
                ViewedSession::Pinned(idx) => assert_eq!(idx.get(), 0),
                ViewedSession::Latest => panic!("Expected Pinned variant"),
            }
        }
    }
}
