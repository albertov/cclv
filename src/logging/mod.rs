//! Tracing subscriber integration for log pane.
//!
//! Provides a custom `tracing_subscriber::Layer` that captures log events
//! and sends them to the UI thread via a channel for display in the log pane.

use crate::state::log_pane::LogPaneEntry;
use std::sync::mpsc;
use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

#[cfg(test)]
#[path = "logging_tests.rs"]
mod tests;

/// A tracing Layer that sends log entries to the UI via a channel.
///
/// This allows tracing output (e.g., `tracing::info!`, `tracing::error!`)
/// to appear in the log pane without blocking the logging thread.
#[allow(dead_code)] // sender used in implementation, not stubs
pub struct LogPaneLayer {
    /// Sender for log entries to the UI thread
    sender: mpsc::Sender<LogPaneEntry>,
}

impl LogPaneLayer {
    /// Create a new LogPaneLayer with the given sender.
    ///
    /// # Arguments
    /// * `sender` - Channel sender for log entries
    ///
    /// # Returns
    /// A new `LogPaneLayer` that will send entries via the provided sender.
    pub fn new(_sender: mpsc::Sender<LogPaneEntry>) -> Self {
        todo!("LogPaneLayer::new")
    }
}

impl<S> Layer<S> for LogPaneLayer
where
    S: Subscriber,
{
    /// Handle a tracing event by converting it to a LogPaneEntry and sending via channel.
    ///
    /// If the channel send fails (receiver dropped), the event is silently ignored
    /// to satisfy FR-059: errors in logging must not break the main UI flow.
    fn on_event(
        &self,
        _event: &tracing::Event<'_>,
        _ctx: Context<'_, S>,
    ) {
        todo!("LogPaneLayer::on_event")
    }
}

/// Initialize the tracing subscriber with a LogPaneLayer.
///
/// This sets up the global default subscriber to send log entries to the UI.
///
/// # Arguments
/// * `sender` - Channel sender for log entries
///
/// # Returns
/// * `Ok(())` if initialization succeeded
/// * `Err(msg)` if the subscriber was already initialized
pub fn init_with_log_pane(_sender: mpsc::Sender<LogPaneEntry>) -> Result<(), String> {
    todo!("init_with_log_pane")
}
