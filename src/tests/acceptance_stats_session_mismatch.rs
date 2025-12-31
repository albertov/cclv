//! Bug reproduction: Stats pane shows global stats instead of session-specific stats
//!
//! EXPECTED: When viewing a specific tab (Main or subagent), the stats pane should
//!           display token counts and costs for that specific session only.
//!
//! ACTUAL: Stats pane shows the same global/aggregated stats regardless of which
//!         tab is currently selected.
//!
//! ## Test 1: Single session with Main + subagent
//!
//! Steps to reproduce manually:
//! 1. cargo run -- tests/fixtures/stats_session_mismatch_repro.jsonl
//! 2. Press 's' to toggle stats pane
//! 3. Observe stats show Input: 3,125 (global total)
//! 4. Press Tab to switch to subagent tab
//! 5. Observe stats STILL show Input: 3,125 (should show ~125 for subagent)
//!
//! Reproduction fixture: tests/fixtures/stats_session_mismatch_repro.jsonl
//! - Main agent: 1000 + 2000 = 3000 input tokens
//! - Subagent: 50 + 75 = 125 input tokens
//! - Global total: 3125 input tokens
//!
//! ## Test 2: Multiple sessions in same log file
//!
//! Steps to reproduce manually:
//! 1. cargo run -- tests/fixtures/stats_multi_session_repro.jsonl
//! 2. Press 's' to toggle stats pane
//! 3. Observe conversation shows Beta session (most recent)
//! 4. Observe stats show Alpha session tokens (2,700) instead of Beta (11,800)
//!
//! Reproduction fixture: tests/fixtures/stats_multi_session_repro.jsonl
//! - Session Alpha: Main (2500) + Subagent (200) = 2700 input tokens
//! - Session Beta: Main (11000) + Subagent (800) = 11800 input tokens
//! - Stats SHOULD show Beta session (displayed), but show Alpha or global

use crate::test_harness::AcceptanceTestHarness;
use crossterm::event::KeyCode;

const STATS_FIXTURE: &str = "tests/fixtures/stats_session_mismatch_repro.jsonl";
const MULTI_SESSION_FIXTURE: &str = "tests/fixtures/stats_multi_session_repro.jsonl";

/// Bug reproduction: stats should change when switching tabs
///
/// When user switches from Main tab to subagent tab, the stats panel should
/// update to show stats for the currently focused session, not global totals.
#[test]
fn bug_stats_should_update_when_switching_tabs() {
    // GIVEN: Viewer loaded with Main (3000 input) and subagent (125 input)
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(STATS_FIXTURE, 100, 30)
        .expect("Should load fixture");

    // Toggle stats pane visible
    harness.send_key(KeyCode::Char('s'));

    // Render with Main tab focused
    let main_tab_output = harness.render_to_string();

    // Snapshot the buggy state (shows global stats: 3,125 input)
    insta::assert_snapshot!("bug_stats_main_tab", main_tab_output);

    // WHEN: User switches to subagent tab
    harness.send_key(KeyCode::Tab);

    // THEN: Stats should show subagent stats (125 input), not global (3,125)
    let subagent_tab_output = harness.render_to_string();

    // Snapshot the buggy state (still shows global stats: 3,125 input)
    insta::assert_snapshot!("bug_stats_subagent_tab", subagent_tab_output);

    // BUG: These two outputs show the SAME stats numbers
    // Expected: Main tab shows ~3,000 input, subagent tab shows ~125 input
    // Actual: Both show 3,125 (global total)

    // This assertion FAILS because the bug exists - stats show global (3,125)
    // instead of subagent-specific (125). When fixed, this will PASS.
    assert!(
        subagent_tab_output.contains("Input:  125") || subagent_tab_output.contains("Input: 125"),
        "BUG: Stats pane shows global stats instead of session-specific stats.\n\
         Expected: Subagent tab should show Input: 125 (subagent tokens only)\n\
         Actual: Stats show Input: 3,125 (global total)\n\
         \n\
         The stats pane does not update when switching tabs.\n\
         Actual output:\n{}",
        subagent_tab_output
    );
}

/// Bug reproduction: multi-session log file shows wrong session stats
///
/// When a log file contains multiple sessions, the conversation pane shows
/// the most recent session (Beta), but the stats pane shows stats from
/// a different session (Alpha) or aggregated across all sessions.
///
/// Expected: Stats should show Beta session tokens (11,800 input)
/// Actual: Stats show Alpha session tokens (2,700 input)
#[test]
fn bug_multi_session_stats_should_match_displayed_session() {
    // GIVEN: Log file with two sessions:
    //   - Alpha: Main (2500) + Subagent (200) = 2700 input
    //   - Beta: Main (11000) + Subagent (800) = 11800 input
    // Conversation pane shows Beta session (most recent)
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(MULTI_SESSION_FIXTURE, 100, 30)
        .expect("Should load multi-session fixture");

    // Toggle stats pane visible
    harness.send_key(KeyCode::Char('s'));

    // WHEN: Rendering the default view (most recent session)
    let output = harness.render_to_string();

    // Snapshot the buggy state
    insta::assert_snapshot!("bug_multi_session_stats", output);

    // Verify conversation shows Beta session content
    assert!(
        output.contains("Beta session"),
        "Conversation pane should show Beta session (most recent)\n\
         Actual output:\n{}",
        output
    );

    // THEN: Stats should show Beta session stats (11,800 input)
    // BUG: Stats show Alpha session (2,700) instead of displayed Beta (11,800)
    assert!(
        output.contains("11,800") || output.contains("11800"),
        "BUG: Stats pane shows wrong session's stats.\n\
         Expected: Stats should show displayed session (Beta: 11,800 input)\n\
         Actual: Stats show different session (Alpha: 2,700 input)\n\
         \n\
         In multi-session logs, stats must match the currently displayed session.\n\
         Actual output:\n{}",
        output
    );
}
