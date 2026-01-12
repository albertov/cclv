//! Keyboard bindings configuration.

use crate::model::key_action::KeyAction;
use crate::state::{FocusPane, SearchState};
use crossterm::event::KeyEvent;
use std::collections::HashMap;

/// Maps keyboard events to domain actions.
///
/// Provides default vim-style bindings with option to override via configuration.
#[derive(Debug, Clone)]
pub struct KeyBindings {
    bindings: HashMap<KeyEvent, KeyAction>,
}

impl KeyBindings {
    /// Look up the action for a key event.
    pub fn get(&self, key: KeyEvent) -> Option<KeyAction> {
        self.bindings.get(&key).copied()
    }

    /// Look up the action for a key event with context awareness.
    ///
    /// Context-aware keybinding resolution supports different actions for the same key
    /// depending on UI state. For example, Enter submits search when in Search + Typing mode,
    /// but toggles message expansion in other contexts.
    ///
    /// # Parameters
    ///
    /// - `key`: The key event to resolve
    /// - `focus`: Which pane currently has focus
    /// - `search_state`: Current search state (Inactive, Typing, or Active)
    ///
    /// # Returns
    ///
    /// The appropriate `KeyAction` for this key in the given context, or `None` if no binding.
    pub fn get_contextual(
        &self,
        key: KeyEvent,
        focus: FocusPane,
        search_state: &SearchState,
    ) -> Option<KeyAction> {
        use crossterm::event::KeyCode;

        // Context override: Enter in Search + Typing → SubmitSearch
        if key.code == KeyCode::Enter
            && key.modifiers.is_empty()
            && focus == FocusPane::Search
            && matches!(search_state, SearchState::Typing { .. })
        {
            return Some(KeyAction::SubmitSearch);
        }

        // Default lookup
        self.bindings.get(&key).copied()
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        use crossterm::event::{KeyCode, KeyModifiers};

        let mut bindings = HashMap::new();

        // Vim-style scrolling
        bindings.insert(
            KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            KeyAction::ScrollDown,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
            KeyAction::ScrollUp,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
            KeyAction::ScrollLeft,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
            KeyAction::ScrollRight,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
            KeyAction::ScrollToTop,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('G'), KeyModifiers::SHIFT),
            KeyAction::ScrollToBottom,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::End, KeyModifiers::NONE),
            KeyAction::ScrollToLatest,
        );

        // Arrow key scrolling
        bindings.insert(
            KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
            KeyAction::ScrollUp,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
            KeyAction::ScrollDown,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Left, KeyModifiers::NONE),
            KeyAction::ScrollLeft,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Right, KeyModifiers::NONE),
            KeyAction::ScrollRight,
        );

        // Page navigation
        bindings.insert(
            KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL),
            KeyAction::PageDown,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL),
            KeyAction::PageUp,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE),
            KeyAction::PageDown,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE),
            KeyAction::PageUp,
        );

        // Tab navigation with Tab key and number keys
        // Note: User presses N to select tab N (1-indexed: 1→tab 1, 2→tab 2, etc.)
        bindings.insert(
            KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
            KeyAction::NextTab,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE),
            KeyAction::SelectTab(1),
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE),
            KeyAction::SelectTab(2),
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE),
            KeyAction::SelectTab(3),
        );

        // Tab navigation
        bindings.insert(
            KeyEvent::new(KeyCode::Char(']'), KeyModifiers::NONE),
            KeyAction::NextTab,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('['), KeyModifiers::NONE),
            KeyAction::PrevTab,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT),
            KeyAction::PrevTab,
        );

        // Direct tab selection (4-9 continue the pattern)
        // Note: User presses N to select tab N (1-indexed)
        bindings.insert(
            KeyEvent::new(KeyCode::Char('4'), KeyModifiers::NONE),
            KeyAction::SelectTab(4),
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('5'), KeyModifiers::NONE),
            KeyAction::SelectTab(5),
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('6'), KeyModifiers::NONE),
            KeyAction::SelectTab(6),
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('7'), KeyModifiers::NONE),
            KeyAction::SelectTab(7),
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('8'), KeyModifiers::NONE),
            KeyAction::SelectTab(8),
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('9'), KeyModifiers::NONE),
            KeyAction::SelectTab(9),
        );

        // Message interaction
        bindings.insert(
            KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            KeyAction::ToggleExpand,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            KeyAction::ToggleExpand,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
            KeyAction::ExpandMessage,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE),
            KeyAction::CollapseMessage,
        );

        // Entry navigation (keyboard focus)
        bindings.insert(
            KeyEvent::new(KeyCode::Char('j'), KeyModifiers::CONTROL),
            KeyAction::NextEntry,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('k'), KeyModifiers::CONTROL),
            KeyAction::PrevEntry,
        );

        // Search
        bindings.insert(
            KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE),
            KeyAction::StartSearch,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('f'), KeyModifiers::CONTROL),
            KeyAction::StartSearch,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE),
            KeyAction::NextMatch,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('N'), KeyModifiers::SHIFT),
            KeyAction::PrevMatch,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            KeyAction::CancelSearch,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL),
            KeyAction::SubmitSearch,
        );

        // Stats
        bindings.insert(
            KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE),
            KeyAction::ToggleStats,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE),
            KeyAction::FilterGlobal,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE),
            KeyAction::FilterMainAgent,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('#'), KeyModifiers::NONE),
            KeyAction::FilterSubagent,
        );

        // Session navigation
        bindings.insert(
            KeyEvent::new(KeyCode::Char('S'), KeyModifiers::SHIFT),
            KeyAction::ToggleSessionModal,
        );

        // Live mode
        bindings.insert(
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
            KeyAction::ToggleAutoScroll,
        );

        // Wrap toggle
        bindings.insert(
            KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE),
            KeyAction::ToggleWrap,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('W'), KeyModifiers::SHIFT),
            KeyAction::ToggleGlobalWrap,
        );

        // Application controls
        bindings.insert(
            KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
            KeyAction::Quit,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE),
            KeyAction::Help,
        );
        bindings.insert(
            KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE),
            KeyAction::Refresh,
        );

        Self { bindings }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    #[test]
    fn default_bindings_map_lowercase_w_to_toggle_wrap() {
        let bindings = KeyBindings::default();
        let key_event = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE);

        assert_eq!(
            bindings.get(key_event),
            Some(KeyAction::ToggleWrap),
            "Lowercase 'w' should map to ToggleWrap"
        );
    }

    #[test]
    fn default_bindings_map_uppercase_w_to_toggle_global_wrap() {
        let bindings = KeyBindings::default();
        let key_event = KeyEvent::new(KeyCode::Char('W'), KeyModifiers::SHIFT);

        assert_eq!(
            bindings.get(key_event),
            Some(KeyAction::ToggleGlobalWrap),
            "Uppercase 'W' (shift+w) should map to ToggleGlobalWrap"
        );
    }

    // ===== Context-Aware Keybinding Tests =====

    #[test]
    fn enter_in_search_typing_mode_returns_submit_search() {
        let kb = KeyBindings::default();
        let enter = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        let typing = SearchState::Typing {
            query: "test".into(),
            cursor: 4,
        };

        assert_eq!(
            kb.get_contextual(enter, FocusPane::Search, &typing),
            Some(KeyAction::SubmitSearch),
            "Enter in Search + Typing mode should submit search"
        );
    }

    #[test]
    fn enter_in_main_focus_returns_toggle_expand() {
        let kb = KeyBindings::default();
        let enter = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        let inactive = SearchState::Inactive;

        assert_eq!(
            kb.get_contextual(enter, FocusPane::Main, &inactive),
            Some(KeyAction::ToggleExpand),
            "Enter in Main focus should toggle message expansion"
        );
    }

    #[test]
    fn enter_in_search_focus_but_inactive_returns_toggle_expand() {
        let kb = KeyBindings::default();
        let enter = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        let inactive = SearchState::Inactive;

        assert_eq!(
            kb.get_contextual(enter, FocusPane::Search, &inactive),
            Some(KeyAction::ToggleExpand),
            "Enter in Search focus but Inactive state should toggle expand"
        );
    }

    #[test]
    fn enter_in_subagent_focus_returns_toggle_expand() {
        let kb = KeyBindings::default();
        let enter = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        let inactive = SearchState::Inactive;

        assert_eq!(
            kb.get_contextual(enter, FocusPane::Subagent, &inactive),
            Some(KeyAction::ToggleExpand),
            "Enter in Subagent focus should toggle expand"
        );
    }

    #[test]
    fn enter_with_modifiers_in_typing_mode_uses_default_binding() {
        let kb = KeyBindings::default();
        let enter_ctrl = KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL);
        let typing = SearchState::Typing {
            query: "test".into(),
            cursor: 4,
        };

        // Ctrl+Enter has no default binding, so should return None
        assert_eq!(
            kb.get_contextual(enter_ctrl, FocusPane::Search, &typing),
            None,
            "Enter with modifiers should not trigger context override"
        );
    }

    #[test]
    fn other_keys_in_typing_mode_use_default_bindings() {
        let kb = KeyBindings::default();
        let typing = SearchState::Typing {
            query: "test".into(),
            cursor: 4,
        };

        // Test that other keys still work normally in typing mode
        let esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        assert_eq!(
            kb.get_contextual(esc, FocusPane::Search, &typing),
            Some(KeyAction::CancelSearch),
            "Esc should cancel search even in typing mode"
        );

        let ctrl_s = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL);
        assert_eq!(
            kb.get_contextual(ctrl_s, FocusPane::Search, &typing),
            Some(KeyAction::SubmitSearch),
            "Ctrl+S should also submit search"
        );
    }
}
