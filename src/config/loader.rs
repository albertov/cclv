//! Configuration file loading with precedence handling.

use serde::Deserialize;
use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during config loading.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// Config file path contains invalid UTF-8 or cannot be resolved.
    #[error("Invalid config path: {0}")]
    InvalidPath(String),

    /// Failed to read config file (file may not exist or have permission issues).
    #[error("Failed to read config file at {path}: {reason}")]
    ReadError {
        /// Path that failed to read.
        path: PathBuf,
        /// Reason for failure.
        reason: String,
    },

    /// Config file contains invalid TOML syntax.
    #[error("Invalid TOML in {path}: {reason}")]
    ParseError {
        /// Path with invalid TOML.
        path: PathBuf,
        /// Parse error details.
        reason: String,
    },
}

/// TOML configuration file structure.
///
/// All fields are optional - if not specified, hardcoded defaults are used.
/// Corresponds to `~/.config/cclv/config.toml`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConfigFile {
    /// Theme name (e.g., "base16-ocean", "solarized-dark").
    #[serde(default)]
    pub theme: Option<String>,

    /// Default follow mode (live tailing).
    #[serde(default)]
    pub follow: Option<bool>,

    /// Show stats panel on startup.
    #[serde(default)]
    pub show_stats: Option<bool>,

    /// Collapse threshold in lines.
    #[serde(default)]
    pub collapse_threshold: Option<usize>,

    /// Summary lines for collapsed messages.
    #[serde(default)]
    pub summary_lines: Option<usize>,

    /// Line wrapping enabled.
    #[serde(default)]
    pub line_wrap: Option<bool>,

    /// Log buffer capacity for logging pane.
    #[serde(default)]
    pub log_buffer_capacity: Option<usize>,

    /// Custom key bindings (future use).
    #[serde(default)]
    pub keybindings: Option<toml::Value>,

    /// Pricing section for cost estimation (future use).
    #[serde(default)]
    pub pricing: Option<toml::Value>,
}

/// Resolved configuration after applying precedence rules.
///
/// Created by merging defaults, config file, env vars, and CLI args.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedConfig {
    /// Theme name.
    pub theme: String,
    /// Follow mode.
    pub follow: bool,
    /// Show stats on startup.
    pub show_stats: bool,
    /// Collapse threshold.
    pub collapse_threshold: usize,
    /// Summary lines.
    pub summary_lines: usize,
    /// Line wrapping.
    pub line_wrap: bool,
    /// Log buffer capacity.
    pub log_buffer_capacity: usize,
}

impl Default for ResolvedConfig {
    fn default() -> Self {
        Self {
            theme: "base16-ocean".to_string(),
            follow: true,
            show_stats: false,
            collapse_threshold: 10,
            summary_lines: 3,
            line_wrap: true,
            log_buffer_capacity: 1000,
        }
    }
}

/// Load configuration file from a specific path.
///
/// Returns `Ok(None)` if file doesn't exist (not an error - use defaults).
/// Returns `Err` if file exists but cannot be read or parsed.
///
/// # Arguments
///
/// * `path` - Path to config file
///
/// # Errors
///
/// Returns error if file exists but has read or parse errors.
pub fn load_config_file(_path: impl Into<PathBuf>) -> Result<Option<ConfigFile>, ConfigError> {
    todo!("load_config_file")
}

/// Resolve default config file path.
///
/// Returns `~/.config/cclv/config.toml` on Unix, appropriate path on other platforms.
/// Returns `None` if home directory cannot be determined.
pub fn default_config_path() -> Option<PathBuf> {
    todo!("default_config_path")
}

/// Load configuration with precedence handling.
///
/// Precedence (highest to lowest):
/// 1. Explicit `config_path` argument (like CLI `--config`)
/// 2. `CCLV_CONFIG` environment variable
/// 3. Default path `~/.config/cclv/config.toml`
///
/// Missing config files are NOT errors - defaults are used.
///
/// # Arguments
///
/// * `config_path` - Optional explicit config path (e.g., from CLI `--config`)
///
/// # Errors
///
/// Returns error only if a config file exists but cannot be read or parsed.
pub fn load_config_with_precedence(
    _config_path: Option<PathBuf>,
) -> Result<Option<ConfigFile>, ConfigError> {
    todo!("load_config_with_precedence")
}

/// Apply environment variable overrides to resolved config.
///
/// Checks for:
/// - `CCLV_THEME`: Override theme
///
/// # Arguments
///
/// * `config` - Base resolved config
///
/// # Returns
///
/// Config with environment overrides applied.
pub fn apply_env_overrides(_config: ResolvedConfig) -> ResolvedConfig {
    todo!("apply_env_overrides")
}

/// Merge config file into defaults to create resolved config.
///
/// For each field in `ConfigFile`, if `Some(value)`, use it; otherwise use default.
///
/// # Arguments
///
/// * `config_file` - Optional loaded config file
///
/// # Returns
///
/// Fully resolved configuration.
pub fn merge_config(_config_file: Option<ConfigFile>) -> ResolvedConfig {
    todo!("merge_config")
}

#[cfg(test)]
#[path = "loader_tests.rs"]
mod tests;
