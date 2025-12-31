//! TUI rendering and terminal management (impure shell)

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};
use thiserror::Error;

/// Errors that can occur during TUI operations
#[derive(Debug, Error)]
pub enum TuiError {
    /// IO error during terminal operations
    #[error("Terminal IO error: {0}")]
    Io(#[from] io::Error),
}

/// Main TUI application
///
/// Generic over backend to support testing with TestBackend
pub struct TuiApp<B>
where
    B: ratatui::backend::Backend,
{
    terminal: Terminal<B>,
}

impl TuiApp<CrosstermBackend<Stdout>> {
    /// Create and initialize a new TUI application
    ///
    /// Sets up terminal in raw mode with alternate screen
    pub fn new() -> Result<Self, TuiError> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    /// Run the main event loop
    ///
    /// Returns when user quits (q or Ctrl+C)
    pub fn run(&mut self) -> Result<(), TuiError> {
        loop {
            self.draw()?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if self.handle_key(key) {
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<B> TuiApp<B>
where
    B: ratatui::backend::Backend,
{
    /// Handle a single keyboard event
    ///
    /// Returns true if app should quit
    fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Quit on 'q' or Ctrl+C
        matches!(key.code, KeyCode::Char('q'))
            || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
    }

    /// Render the current frame
    fn draw(&mut self) -> Result<(), TuiError> {
        self.terminal.draw(|_frame| {
            // Blank screen for now - just clear the terminal
        })?;
        Ok(())
    }
}

/// Initialize and run the TUI application
///
/// This is the main entry point for the TUI. It handles terminal
/// setup, runs the event loop, and ensures cleanup on exit.
pub fn run() -> Result<(), TuiError> {
    let mut app = TuiApp::new()?;

    // Run the app and ensure cleanup happens even on error
    let result = app.run();

    // Always restore terminal state
    restore_terminal()?;

    result
}

/// Restore terminal to normal state
///
/// Disables raw mode and leaves alternate screen
fn restore_terminal() -> Result<(), TuiError> {
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;

    #[test]
    fn tui_error_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::Other, "test error");
        let tui_err: TuiError = io_err.into();
        assert!(matches!(tui_err, TuiError::Io(_)));
    }

    #[test]
    fn handle_key_q_returns_true() {
        // Create a mock TUI app using TestBackend
        use ratatui::backend::TestBackend;
        let backend = TestBackend::new(80, 24);
        let terminal = Terminal::new(backend).unwrap();

        let mut app = TuiApp { terminal };

        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        let should_quit = app.handle_key(key);

        assert!(should_quit, "'q' should trigger quit");
    }

    #[test]
    fn handle_key_ctrl_c_returns_true() {
        use ratatui::backend::TestBackend;
        let backend = TestBackend::new(80, 24);
        let terminal = Terminal::new(backend).unwrap();

        let mut app = TuiApp { terminal };

        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        let should_quit = app.handle_key(key);

        assert!(should_quit, "Ctrl+C should trigger quit");
    }

    #[test]
    fn handle_key_other_returns_false() {
        use ratatui::backend::TestBackend;
        let backend = TestBackend::new(80, 24);
        let terminal = Terminal::new(backend).unwrap();

        let mut app = TuiApp { terminal };

        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        let should_quit = app.handle_key(key);

        assert!(!should_quit, "Normal keys should not trigger quit");
    }

    #[test]
    fn draw_renders_without_error() {
        use ratatui::backend::TestBackend;
        let backend = TestBackend::new(80, 24);
        let terminal = Terminal::new(backend).unwrap();

        let mut app = TuiApp { terminal };

        let result = app.draw();
        assert!(result.is_ok(), "Drawing should succeed");
    }
}
