//! Configuration module.

pub mod keybindings;

pub use keybindings::KeyBindings;

/// Application-level configuration.
///
/// Holds global settings that affect application behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    /// Whether line wrapping is enabled globally.
    ///
    /// When `true`, long lines wrap to fit the viewport width.
    /// When `false`, long lines require horizontal scrolling.
    pub line_wrap: bool,

    /// Maximum log entries to retain in logging pane.
    ///
    /// Ring buffer capacity for the logging pane. When full, oldest entries
    /// are dropped. Default: 1000 entries (FR-056).
    pub log_buffer_capacity: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            line_wrap: true,
            log_buffer_capacity: 1000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_line_wrap_enabled() {
        let config = AppConfig::default();
        assert!(
            config.line_wrap,
            "Default config should have line_wrap=true per FR-039"
        );
    }

    #[test]
    fn default_config_is_cloneable() {
        let config = AppConfig::default();
        let cloned = config.clone();
        assert_eq!(config, cloned, "Cloned config should equal original");
    }

    #[test]
    fn can_create_config_with_wrap_disabled() {
        let config = AppConfig {
            line_wrap: false,
            log_buffer_capacity: 1000,
        };
        assert!(!config.line_wrap, "Should allow line_wrap=false");
    }

    #[test]
    fn can_create_config_with_wrap_enabled() {
        let config = AppConfig {
            line_wrap: true,
            log_buffer_capacity: 1000,
        };
        assert!(config.line_wrap, "Should allow line_wrap=true");
    }

    #[test]
    fn default_config_has_log_buffer_capacity_1000() {
        let config = AppConfig::default();
        assert_eq!(
            config.log_buffer_capacity, 1000,
            "Default config should have log_buffer_capacity=1000 per FR-056"
        );
    }

    #[test]
    fn can_create_config_with_custom_log_buffer_capacity() {
        let config = AppConfig {
            line_wrap: true,
            log_buffer_capacity: 500,
        };
        assert_eq!(
            config.log_buffer_capacity, 500,
            "Should allow custom log_buffer_capacity"
        );
    }
}
