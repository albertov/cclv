//! Stdin-based log source for piped input.
//!
//! Provides StdinSource for reading JSONL from stdin with support for
//! both streaming (live) and complete (EOF reached) modes.

use crate::model::error::InputError;
use std::io::{BufRead, BufReader, IsTerminal, Read};

/// Stdin source for piped JSONL input.
///
/// Supports both streaming mode (data arriving incrementally, like `tail -f | cclv`)
/// and complete mode (EOF reached, like `cat file.jsonl | cclv`).
///
/// # Design
///
/// - Detects TTY vs piped input at construction
/// - Non-blocking poll() for TUI event loop integration
/// - Tracks EOF state via `complete` flag
pub struct StdinSource<R: Read> {
    reader: BufReader<R>,
    complete: bool,
}

impl StdinSource<std::io::Stdin> {
    /// Create a new StdinSource from stdin.
    ///
    /// # Errors
    ///
    /// Returns `InputError::NoInput` if stdin is a TTY (interactive terminal).
    /// This prevents the TUI from blocking waiting for user input when the
    /// user forgot to pipe data.
    pub fn new() -> Result<Self, InputError> {
        if Self::is_tty() {
            return Err(InputError::NoInput);
        }

        Ok(Self {
            reader: BufReader::new(std::io::stdin()),
            complete: false,
        })
    }

    /// Check if stdin is a TTY (interactive terminal).
    ///
    /// Used internally to detect piped vs interactive stdin.
    fn is_tty() -> bool {
        std::io::stdin().is_terminal()
    }
}

impl<R: Read> StdinSource<R> {
    /// Create StdinSource from any reader (for testing).
    ///
    /// Internal constructor - bypasses TTY check for testing.
    #[cfg(test)]
    fn from_reader(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            complete: false,
        }
    }

    /// Poll for a new line from stdin.
    ///
    /// Non-blocking: returns immediately with `None` if no complete line available.
    /// Returns `Some(line)` when a complete line is read.
    ///
    /// Sets `complete` flag to true when EOF is reached.
    ///
    /// # Errors
    ///
    /// Returns `InputError::Io` for I/O errors.
    pub fn poll(&mut self) -> Result<Option<String>, InputError> {
        let mut buffer = String::new();
        let bytes_read = self.reader.read_line(&mut buffer)?;

        if bytes_read == 0 {
            // EOF reached
            self.complete = true;
            return Ok(None);
        }

        // Only return complete lines (ending with newline)
        if buffer.ends_with('\n') {
            // Remove trailing newline
            buffer.truncate(buffer.trim_end().len());
            Ok(Some(buffer))
        } else {
            // Partial line - return None but don't mark complete
            Ok(None)
        }
    }

    /// Check if EOF has been reached (no more data will arrive).
    pub fn is_complete(&self) -> bool {
        self.complete
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_returns_line_when_data_available() {
        let data = b"{\"line\": 1}\n{\"line\": 2}\n";
        let mut source = StdinSource::from_reader(&data[..]);

        let line1 = source.poll().unwrap();
        assert_eq!(line1, Some("{\"line\": 1}".to_string()));

        let line2 = source.poll().unwrap();
        assert_eq!(line2, Some("{\"line\": 2}".to_string()));
    }

    #[test]
    fn poll_returns_none_at_eof() {
        let data = b"{\"line\": 1}\n";
        let mut source = StdinSource::from_reader(&data[..]);

        // Read the one line
        source.poll().unwrap();

        // Next poll should return None and set complete flag
        let result = source.poll().unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn is_complete_true_after_eof() {
        let data = b"{\"line\": 1}\n";
        let mut source = StdinSource::from_reader(&data[..]);

        assert!(!source.is_complete(), "Should not be complete initially");

        // Read until EOF
        source.poll().unwrap();
        source.poll().unwrap();

        assert!(source.is_complete(), "Should be complete after EOF");
    }

    #[test]
    fn poll_handles_partial_lines() {
        // Data without trailing newline
        let data = b"{\"line\": 1}\n{\"partial";
        let mut source = StdinSource::from_reader(&data[..]);

        // Should read the complete line
        let line1 = source.poll().unwrap();
        assert_eq!(line1, Some("{\"line\": 1}".to_string()));

        // Should return None for partial line (not complete)
        let line2 = source.poll().unwrap();
        assert_eq!(line2, None);
    }

    #[test]
    fn poll_returns_none_for_empty_input() {
        let data = b"";
        let mut source = StdinSource::from_reader(&data[..]);

        let result = source.poll().unwrap();
        assert_eq!(result, None);
        assert!(source.is_complete(), "Empty input should be complete");
    }

    #[test]
    fn poll_strips_newline_from_result() {
        let data = b"line with newline\n";
        let mut source = StdinSource::from_reader(&data[..]);

        let line = source.poll().unwrap();
        assert_eq!(line, Some("line with newline".to_string()));
        assert!(!line.unwrap().contains('\n'), "Should not include newline");
    }

    #[test]
    fn poll_handles_multiple_consecutive_calls() {
        let data = b"line1\nline2\nline3\n";
        let mut source = StdinSource::from_reader(&data[..]);

        assert_eq!(source.poll().unwrap(), Some("line1".to_string()));
        assert_eq!(source.poll().unwrap(), Some("line2".to_string()));
        assert_eq!(source.poll().unwrap(), Some("line3".to_string()));
        assert_eq!(source.poll().unwrap(), None);
        assert!(source.is_complete());
    }
}
