//! Claude Code Log Viewer - Entry Point

use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() {
    // Initialize tracing subscriber with env filter
    // Default to info level, override with RUST_LOG env var
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    info!(version = env!("CARGO_PKG_VERSION"), "cclv starting");
}
