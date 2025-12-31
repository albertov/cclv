//! Mouse hit-testing results

use super::types::EntryIndex;

/// Result of hit-testing a screen coordinate.
///
/// Determines what entry (if any) was clicked and where.
/// Uses `EntryIndex` as the canonical reference for entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HitTestResult {
    /// Click was outside any entry bounds.
    Miss,

    /// Click hit an entry.
    Hit {
        /// Index of the hit entry (canonical reference).
        entry_index: EntryIndex,
        /// Line within the entry that was hit (0-indexed).
        line_in_entry: usize,
        /// Column within the line (0-indexed).
        column: u16,
    },
}

impl HitTestResult {
    /// Create a miss result.
    pub fn miss() -> Self {
        Self::Miss
    }

    /// Create a hit result.
    pub fn hit(entry_index: EntryIndex, line_in_entry: usize, column: u16) -> Self {
        Self::Hit {
            entry_index,
            line_in_entry,
            column,
        }
    }

    /// Check if this was a hit.
    pub fn is_hit(&self) -> bool {
        matches!(self, Self::Hit { .. })
    }

    /// Get entry index if hit.
    pub fn entry_index(&self) -> Option<EntryIndex> {
        match self {
            Self::Hit { entry_index, .. } => Some(*entry_index),
            Self::Miss => None,
        }
    }
}
