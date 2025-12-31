//! Tests for tab navigation handler.
//!
//! Tests verify that tab actions are correctly dispatched to AppState methods:
//! - NextTab moves to next tab (with wrapping)
//! - PrevTab moves to previous tab (with wrapping)
//! - SelectTab(n) selects tab by 1-indexed number
//! - All actions respect focus (only work when Subagent pane focused)
//! - All actions handle edge cases (no subagents, out of bounds)

use super::*;
use crate::model::{
    AgentId, ConversationEntry, EntryMetadata, EntryType, EntryUuid, LogEntry, Message,
    MessageContent, Role, SessionId,
};
use crate::state::{AppState, FocusPane};
use chrono::Utc;

// ===== Test Helpers =====

fn make_session_id(s: &str) -> SessionId {
    SessionId::new(s).expect("valid session id")
}

fn make_entry_uuid(s: &str) -> EntryUuid {
    EntryUuid::new(s).expect("valid uuid")
}

fn make_subagent_entry(agent_id: &str) -> ConversationEntry {
    let log_entry = LogEntry::new(
        make_entry_uuid(&format!("entry-{}", agent_id)),
        None,
        make_session_id("test-session"),
        Some(AgentId::new(agent_id).expect("valid agent id")),
        Utc::now(),
        EntryType::Assistant,
        Message::new(
            Role::Assistant,
            MessageContent::Text("Test message".to_string()),
        ),
        EntryMetadata::default(),
    );

    ConversationEntry::Valid(Box::new(log_entry))
}

// ===== NextTab tests =====

#[test]
fn next_tab_moves_to_next_tab() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));
    entries.push(make_subagent_entry("agent-2"));
    entries.push(make_subagent_entry("agent-3"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;
    state.selected_tab = Some(0);

    let new_state = handle_tab_action(state, KeyAction::NextTab);

    assert_eq!(
        new_state.selected_tab,
        Some(1),
        "NextTab should move from tab 0 to tab 1"
    );
}

#[test]
fn next_tab_wraps_from_last_to_first() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));
    entries.push(make_subagent_entry("agent-2"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;
    state.selected_tab = Some(1); // Last tab

    let new_state = handle_tab_action(state, KeyAction::NextTab);

    assert_eq!(
        new_state.selected_tab,
        Some(0),
        "NextTab should wrap from last tab to first"
    );
}

#[test]
fn next_tab_initializes_to_first_when_none() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;
    state.selected_tab = None;

    let new_state = handle_tab_action(state, KeyAction::NextTab);

    assert_eq!(
        new_state.selected_tab,
        Some(0),
        "NextTab should initialize to first tab when None"
    );
}

#[test]
fn next_tab_does_nothing_when_focus_not_on_subagent() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));
    entries.push(make_subagent_entry("agent-2"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Main;
    state.selected_tab = Some(0);

    let new_state = handle_tab_action(state, KeyAction::NextTab);

    assert_eq!(
        new_state.selected_tab,
        Some(0),
        "NextTab should not change tab when focus is not on Subagent"
    );
}

#[test]
fn next_tab_does_nothing_when_no_subagents() {
    let mut state = AppState::new();
    state.focus = FocusPane::Subagent;
    state.selected_tab = None;

    let new_state = handle_tab_action(state, KeyAction::NextTab);

    assert_eq!(
        new_state.selected_tab, None,
        "NextTab should not change tab when no subagents exist"
    );
}

// ===== PrevTab tests =====

#[test]
fn prev_tab_moves_to_previous_tab() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));
    entries.push(make_subagent_entry("agent-2"));
    entries.push(make_subagent_entry("agent-3"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;
    state.selected_tab = Some(2); // Third tab

    let new_state = handle_tab_action(state, KeyAction::PrevTab);

    assert_eq!(
        new_state.selected_tab,
        Some(1),
        "PrevTab should move from tab 2 to tab 1"
    );
}

#[test]
fn prev_tab_wraps_from_first_to_last() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));
    entries.push(make_subagent_entry("agent-2"));
    entries.push(make_subagent_entry("agent-3"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;
    state.selected_tab = Some(0); // First tab

    let new_state = handle_tab_action(state, KeyAction::PrevTab);

    assert_eq!(
        new_state.selected_tab,
        Some(2),
        "PrevTab should wrap from first tab to last (index 2)"
    );
}

#[test]
fn prev_tab_initializes_to_first_when_none() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;
    state.selected_tab = None;

    let new_state = handle_tab_action(state, KeyAction::PrevTab);

    assert_eq!(
        new_state.selected_tab,
        Some(0),
        "PrevTab should initialize to first tab when None"
    );
}

#[test]
fn prev_tab_does_nothing_when_focus_not_on_subagent() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));
    entries.push(make_subagent_entry("agent-2"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Stats;
    state.selected_tab = Some(1);

    let new_state = handle_tab_action(state, KeyAction::PrevTab);

    assert_eq!(
        new_state.selected_tab,
        Some(1),
        "PrevTab should not change tab when focus is not on Subagent"
    );
}

#[test]
fn prev_tab_does_nothing_when_no_subagents() {
    let mut state = AppState::new();
    state.focus = FocusPane::Subagent;
    state.selected_tab = None;

    let new_state = handle_tab_action(state, KeyAction::PrevTab);

    assert_eq!(
        new_state.selected_tab, None,
        "PrevTab should not change tab when no subagents exist"
    );
}

// ===== SelectTab tests =====

#[test]
fn select_tab_sets_tab_by_one_indexed_number() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));
    entries.push(make_subagent_entry("agent-2"));
    entries.push(make_subagent_entry("agent-3"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;
    state.selected_tab = Some(0);

    let new_state = handle_tab_action(state, KeyAction::SelectTab(2));

    assert_eq!(
        new_state.selected_tab,
        Some(1),
        "SelectTab(2) should select second tab (0-indexed as 1)"
    );
}

#[test]
fn select_tab_handles_tab_1() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;
    state.selected_tab = None;

    let new_state = handle_tab_action(state, KeyAction::SelectTab(1));

    assert_eq!(
        new_state.selected_tab,
        Some(0),
        "SelectTab(1) should select first tab (0-indexed as 0)"
    );
}

#[test]
fn select_tab_clamps_to_last_when_too_high() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));
    entries.push(make_subagent_entry("agent-2"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;
    state.selected_tab = Some(0);

    let new_state = handle_tab_action(state, KeyAction::SelectTab(9));

    assert_eq!(
        new_state.selected_tab,
        Some(1),
        "SelectTab(9) should clamp to last tab when number is too high"
    );
}

#[test]
fn select_tab_ignores_zero() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));
    entries.push(make_subagent_entry("agent-2"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;
    state.selected_tab = Some(1);

    let new_state = handle_tab_action(state, KeyAction::SelectTab(0));

    assert_eq!(
        new_state.selected_tab,
        Some(1),
        "SelectTab(0) should be ignored (invalid 1-indexed input)"
    );
}

#[test]
fn select_tab_does_nothing_when_focus_not_on_subagent() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));
    entries.push(make_subagent_entry("agent-2"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Main;
    state.selected_tab = Some(0);

    let new_state = handle_tab_action(state, KeyAction::SelectTab(2));

    assert_eq!(
        new_state.selected_tab,
        Some(0),
        "SelectTab should not change tab when focus is not on Subagent"
    );
}

#[test]
fn select_tab_does_nothing_when_no_subagents() {
    let mut state = AppState::new();
    state.focus = FocusPane::Subagent;
    state.selected_tab = None;

    let new_state = handle_tab_action(state, KeyAction::SelectTab(1));

    assert_eq!(
        new_state.selected_tab, None,
        "SelectTab should not change tab when no subagents exist"
    );
}

// ===== Non-tab action tests =====

#[test]
fn non_tab_actions_return_state_unchanged() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));
    entries.push(make_subagent_entry("agent-2"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;
    state.selected_tab = Some(1);

    let new_state = handle_tab_action(state, KeyAction::ScrollDown);

    assert_eq!(
        new_state.selected_tab,
        Some(1),
        "Non-tab actions should return state unchanged"
    );
}

#[test]
fn non_tab_actions_like_quit_return_state_unchanged() {
    let mut entries = Vec::new();
    entries.push(make_subagent_entry("agent-1"));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;
    state.selected_tab = Some(0);

    let new_state = handle_tab_action(state, KeyAction::Quit);

    assert_eq!(
        new_state.selected_tab,
        Some(0),
        "Quit action should return state unchanged"
    );
}

// ===== Multi-session tab scoping tests (FR-080, FR-081) =====

/// Helper to create a main conversation entry for a session
fn make_main_entry(session_id: &str, content: &str) -> ConversationEntry {
    let log_entry = LogEntry::new(
        make_entry_uuid(&format!("main-{}", session_id)),
        None,
        make_session_id(session_id),
        None, // Main agent has no agent_id
        Utc::now(),
        EntryType::User,
        Message::new(Role::User, MessageContent::Text(content.to_string())),
        EntryMetadata::default(),
    );

    ConversationEntry::Valid(Box::new(log_entry))
}

/// Helper to create a subagent entry for a specific session and agent
fn make_subagent_entry_for_session(
    session_id: &str,
    agent_id: &str,
    content: &str,
) -> ConversationEntry {
    let log_entry = LogEntry::new(
        make_entry_uuid(&format!("entry-{}-{}", session_id, agent_id)),
        None,
        make_session_id(session_id),
        Some(AgentId::new(agent_id).expect("valid agent id")),
        Utc::now(),
        EntryType::Assistant,
        Message::new(Role::Assistant, MessageContent::Text(content.to_string())),
        EntryMetadata::default(),
    );

    ConversationEntry::Valid(Box::new(log_entry))
}

#[test]
fn next_tab_uses_active_session_subagents_when_scrolled_to_first_session() {
    // Given: Two sessions with different subagent sets
    // Session 1: alpha, beta
    // Session 2: gamma, delta, epsilon
    let mut entries = Vec::new();

    // Session 1
    entries.push(make_main_entry("session-1", "First session"));
    entries.push(make_subagent_entry_for_session(
        "session-1",
        "alpha",
        "Alpha msg",
    ));
    entries.push(make_subagent_entry_for_session(
        "session-1",
        "beta",
        "Beta msg",
    ));

    // Session 2
    entries.push(make_main_entry("session-2", "Second session"));
    entries.push(make_subagent_entry_for_session(
        "session-2",
        "gamma",
        "Gamma msg",
    ));
    entries.push(make_subagent_entry_for_session(
        "session-2",
        "delta",
        "Delta msg",
    ));
    entries.push(make_subagent_entry_for_session(
        "session-2",
        "epsilon",
        "Epsilon msg",
    ));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;

    // CRITICAL: Scroll position determines active session
    // Scroll line 0 = session 1 is active
    // Session 1 has subagents: alpha (0), beta (1)
    state.selected_tab = Some(0); // alpha selected

    let new_state = handle_tab_action(state, KeyAction::NextTab);

    // Should wrap within session 1's 2 subagents: alpha (0) -> beta (1)
    assert_eq!(
        new_state.selected_tab,
        Some(1),
        "NextTab from alpha should select beta (within session 1's subagents)"
    );
}

#[test]
fn next_tab_wraps_within_active_session_subagents() {
    // Given: Two sessions with different subagent counts
    let mut entries = Vec::new();

    // Session 1: alpha, beta (2 subagents)
    entries.push(make_main_entry("session-1", "First session"));
    entries.push(make_subagent_entry_for_session(
        "session-1",
        "alpha",
        "Alpha msg",
    ));
    entries.push(make_subagent_entry_for_session(
        "session-1",
        "beta",
        "Beta msg",
    ));

    // Session 2: gamma, delta, epsilon (3 subagents)
    entries.push(make_main_entry("session-2", "Second session"));
    entries.push(make_subagent_entry_for_session(
        "session-2",
        "gamma",
        "Gamma msg",
    ));
    entries.push(make_subagent_entry_for_session(
        "session-2",
        "delta",
        "Delta msg",
    ));
    entries.push(make_subagent_entry_for_session(
        "session-2",
        "epsilon",
        "Epsilon msg",
    ));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;

    // When scrolled to session 1, at last tab (beta = index 1)
    state.selected_tab = Some(1); // beta (last in session 1)

    let new_state = handle_tab_action(state, KeyAction::NextTab);

    // Should wrap back to first tab in session 1 (alpha = index 0)
    // NOT continue to session 2's subagents
    assert_eq!(
        new_state.selected_tab,
        Some(0),
        "NextTab from beta (last in session 1) should wrap to alpha (first in session 1)"
    );
}

#[test]
fn prev_tab_uses_active_session_subagents() {
    // Given: Two sessions
    let mut entries = Vec::new();

    // Session 1: alpha, beta
    entries.push(make_main_entry("session-1", "First session"));
    entries.push(make_subagent_entry_for_session(
        "session-1",
        "alpha",
        "Alpha msg",
    ));
    entries.push(make_subagent_entry_for_session(
        "session-1",
        "beta",
        "Beta msg",
    ));

    // Session 2: gamma, delta
    entries.push(make_main_entry("session-2", "Second session"));
    entries.push(make_subagent_entry_for_session(
        "session-2",
        "gamma",
        "Gamma msg",
    ));
    entries.push(make_subagent_entry_for_session(
        "session-2",
        "delta",
        "Delta msg",
    ));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;

    // Scrolled to session 1, at first tab (alpha = index 0)
    state.selected_tab = Some(0); // alpha

    let new_state = handle_tab_action(state, KeyAction::PrevTab);

    // Should wrap to last tab in session 1 (beta = index 1)
    // Session 1 has 2 subagents, so last is index 1
    assert_eq!(
        new_state.selected_tab,
        Some(1),
        "PrevTab from alpha (first in session 1) should wrap to beta (last in session 1)"
    );
}

#[test]
fn select_tab_clamps_to_active_session_subagent_count() {
    // Given: Two sessions with different subagent counts
    let mut entries = Vec::new();

    // Session 1: alpha, beta (2 subagents)
    entries.push(make_main_entry("session-1", "First session"));
    entries.push(make_subagent_entry_for_session(
        "session-1",
        "alpha",
        "Alpha msg",
    ));
    entries.push(make_subagent_entry_for_session(
        "session-1",
        "beta",
        "Beta msg",
    ));

    // Session 2: gamma, delta, epsilon (3 subagents)
    entries.push(make_main_entry("session-2", "Second session"));
    entries.push(make_subagent_entry_for_session(
        "session-2",
        "gamma",
        "Gamma msg",
    ));
    entries.push(make_subagent_entry_for_session(
        "session-2",
        "delta",
        "Delta msg",
    ));
    entries.push(make_subagent_entry_for_session(
        "session-2",
        "epsilon",
        "Epsilon msg",
    ));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;

    // Scrolled to session 1 (which has only 2 subagents)
    state.selected_tab = Some(0);

    let new_state = handle_tab_action(state, KeyAction::SelectTab(5));

    // Should clamp to last tab in session 1 (beta = index 1)
    // NOT to session 2's higher count
    assert_eq!(
        new_state.selected_tab,
        Some(1),
        "SelectTab(5) in session 1 should clamp to last tab in session 1 (index 1)"
    );
}

#[test]
fn tab_operations_respect_scroll_position_to_determine_active_session() {
    // This test verifies the CRITICAL requirement: scroll position determines active session
    // Given: Two sessions with DIFFERENT subagent sets
    let mut entries = Vec::new();

    // Session 1: alpha, beta (2 subagents)
    entries.push(make_main_entry("session-1", "First session"));
    entries.push(make_subagent_entry_for_session(
        "session-1",
        "alpha",
        "Alpha msg",
    ));
    entries.push(make_subagent_entry_for_session(
        "session-1",
        "beta",
        "Beta msg",
    ));

    // Session 2: gamma (1 subagent)
    entries.push(make_main_entry("session-2", "Second session"));
    entries.push(make_subagent_entry_for_session(
        "session-2",
        "gamma",
        "Gamma msg",
    ));

    let mut state = AppState::new();
    state.add_entries(entries);
    state.focus = FocusPane::Subagent;

    // Verify multi-session state was created
    assert_eq!(
        state.log_view().session_count(),
        2,
        "Should have created 2 sessions"
    );

    // Verify session 1 has 2 subagents
    let session1_subagent_count = state
        .log_view()
        .get_session(0)
        .unwrap()
        .subagent_ids()
        .count();
    assert_eq!(
        session1_subagent_count, 2,
        "Session 1 should have 2 subagents"
    );

    // Verify session 2 has 1 subagent
    let session2_subagent_count = state
        .log_view()
        .get_session(1)
        .unwrap()
        .subagent_ids()
        .count();
    assert_eq!(
        session2_subagent_count, 1,
        "Session 2 should have 1 subagent"
    );

    // When scrolled to session 2 (scroll position beyond session 1's content)
    // Session 2 only has gamma (1 subagent)
    // So NextTab from tab 0 (gamma) should wrap back to tab 0 (gamma)
    // NOT to tab 1 (which would be beta from session 1)

    // TODO: This test cannot currently set scroll position directly.
    // It would need to:
    // 1. Get the main conversation view state
    // 2. Calculate session 2's start line
    // 3. Set scroll position to that line
    //
    // For now, this test documents the EXPECTED behavior that
    // tab operations should consider scroll position via active_session().
}
