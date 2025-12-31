//! Acceptance tests for scroll functionality
//!
//! Verifies that users can scroll the conversation view using keyboard commands.
//! These tests verify the fix for bug cclv-5ur.7 where scroll keys had no effect.
//!
//! Test scenarios:
//! 1. User can scroll down with 'j' key
//! 2. User can scroll up with 'k' key
//! 3. User can jump to bottom with 'G' key
//! 4. User can jump to top with 'g' key
//! 5. User can page down with Page Down
//! 6. User can page up with Page Up

use crate::test_harness::AcceptanceTestHarness;
use crossterm::event::KeyCode;

// ===== Test Fixtures =====

/// Fixture with enough entries to require scrolling (20 entries)
const SCROLL_FIXTURE: &str = "tests/fixtures/blank_lines_repro.jsonl";

// ===== Scroll Down Tests =====

#[test]
fn scroll_down_with_j_key_changes_viewport() {
    // GIVEN: Viewer showing conversation at top
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(SCROLL_FIXTURE, 80, 24)
        .expect("Should load fixture");

    // Render initial state
    let initial_output = harness.render_to_string();

    // WHEN: User presses 'j' to scroll down
    harness.send_key(KeyCode::Char('j'));

    // THEN: Viewport content changes (scrolled down by 1 line)
    let scrolled_output = harness.render_to_string();

    assert_ne!(
        initial_output, scrolled_output,
        "Pressing 'j' should change viewport content by scrolling down 1 line"
    );

    // Verify with snapshot
    insta::assert_snapshot!("scroll_down_j_key", scrolled_output);
}

#[test]
fn scroll_down_multiple_times_continues_scrolling() {
    // GIVEN: Viewer at top
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(SCROLL_FIXTURE, 80, 24)
        .expect("Should load fixture");

    harness.render_to_string();
    let initial_output = harness.render_to_string();

    // WHEN: User presses 'j' three times
    harness.send_key(KeyCode::Char('j'));
    harness.send_key(KeyCode::Char('j'));
    harness.send_key(KeyCode::Char('j'));

    // THEN: Viewport shows content further down than initial
    let scrolled_output = harness.render_to_string();

    assert_ne!(
        initial_output, scrolled_output,
        "Multiple 'j' presses should scroll viewport down progressively"
    );

    // Verify with snapshot
    insta::assert_snapshot!("scroll_down_j_key_3x", scrolled_output);
}

// ===== Scroll Up Tests =====

#[test]
fn scroll_up_with_k_key_changes_viewport() {
    // GIVEN: Viewer scrolled down from top
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(SCROLL_FIXTURE, 80, 24)
        .expect("Should load fixture");

    // Scroll down using Page Down to ensure we're past the first screen
    // (Use 1x PageDown to stay within scrollable range)
    harness.send_key(KeyCode::PageDown);

    let scrolled_down_output = harness.render_to_string();

    // WHEN: User presses 'k' to scroll up
    harness.send_key(KeyCode::Char('k'));

    // THEN: Viewport content changes (scrolled up by 1 line)
    let scrolled_up_output = harness.render_to_string();

    assert_ne!(
        scrolled_down_output, scrolled_up_output,
        "Pressing 'k' should change viewport content by scrolling up 1 line"
    );

    // Verify with snapshot
    insta::assert_snapshot!("scroll_up_k_key", scrolled_up_output);
}

#[test]
fn scroll_up_multiple_times_continues_scrolling() {
    // GIVEN: Viewer scrolled down
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(SCROLL_FIXTURE, 80, 24)
        .expect("Should load fixture");

    harness.render_to_string();

    // Scroll down to middle
    for _ in 0..20 {
        harness.send_key(KeyCode::Char('j'));
    }

    let middle_output = harness.render_to_string();

    // WHEN: User presses 'k' five times
    for _ in 0..5 {
        harness.send_key(KeyCode::Char('k'));
    }

    // THEN: Viewport shows content higher up than middle
    let scrolled_up_output = harness.render_to_string();

    assert_ne!(
        middle_output, scrolled_up_output,
        "Multiple 'k' presses should scroll viewport up progressively"
    );

    // Verify with snapshot
    insta::assert_snapshot!("scroll_up_k_key_5x", scrolled_up_output);
}

// ===== Jump to Bottom Tests =====

#[test]
fn jump_to_bottom_with_shift_g_shows_last_entries() {
    // GIVEN: Viewer at top
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(SCROLL_FIXTURE, 80, 24)
        .expect("Should load fixture");

    harness.render_to_string();
    let top_output = harness.render_to_string();

    // WHEN: User presses Shift+G to jump to bottom
    harness.send_key_with_mods(KeyCode::Char('G'), crossterm::event::KeyModifiers::SHIFT);

    // THEN: Viewport shows last entries (different from top)
    let bottom_output = harness.render_to_string();

    assert_ne!(
        top_output, bottom_output,
        "Pressing Shift+G should jump to bottom, showing last entries"
    );

    // Verify with snapshot
    insta::assert_snapshot!("jump_to_bottom_shift_g", bottom_output);
}

// ===== Jump to Top Tests =====

#[test]
fn jump_to_top_with_g_shows_first_entries() {
    // GIVEN: Viewer at bottom
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(SCROLL_FIXTURE, 80, 24)
        .expect("Should load fixture");

    harness.render_to_string();

    // Jump to bottom first
    harness.send_key_with_mods(KeyCode::Char('G'), crossterm::event::KeyModifiers::SHIFT);
    let bottom_output = harness.render_to_string();

    // WHEN: User presses 'g' to jump to top
    harness.send_key(KeyCode::Char('g'));

    // THEN: Viewport shows first entries (different from bottom)
    let top_output = harness.render_to_string();

    assert_ne!(
        bottom_output, top_output,
        "Pressing 'g' should jump to top, showing first entries"
    );

    // Verify with snapshot
    insta::assert_snapshot!("jump_to_top_g", top_output);
}

// ===== Page Down Tests =====

#[test]
fn page_down_scrolls_by_viewport_height() {
    // GIVEN: Viewer at top
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(SCROLL_FIXTURE, 80, 24)
        .expect("Should load fixture");

    harness.render_to_string();
    let top_output = harness.render_to_string();

    // WHEN: User presses Page Down
    harness.send_key(KeyCode::PageDown);

    // THEN: Viewport scrolls down by approximately viewport height
    let paged_output = harness.render_to_string();

    assert_ne!(
        top_output, paged_output,
        "Pressing Page Down should scroll viewport by viewport height"
    );

    // Verify with snapshot
    insta::assert_snapshot!("page_down", paged_output);
}

#[test]
fn page_down_multiple_times_continues_scrolling() {
    // GIVEN: Viewer at top
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(SCROLL_FIXTURE, 80, 24)
        .expect("Should load fixture");

    harness.render_to_string();

    // Press Page Down once
    harness.send_key(KeyCode::PageDown);
    let first_page_output = harness.render_to_string();

    // WHEN: User presses Page Down again
    harness.send_key(KeyCode::PageDown);

    // THEN: Viewport scrolls down further
    let second_page_output = harness.render_to_string();

    assert_ne!(
        first_page_output, second_page_output,
        "Multiple Page Down presses should continue scrolling down"
    );

    // Verify with snapshot
    insta::assert_snapshot!("page_down_2x", second_page_output);
}

// ===== Page Up Tests =====

#[test]
fn page_up_scrolls_by_viewport_height() {
    // GIVEN: Viewer scrolled down
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(SCROLL_FIXTURE, 80, 24)
        .expect("Should load fixture");

    harness.render_to_string();

    // Scroll down several pages (use 2x to stay within scrollable range)
    for _ in 0..2 {
        harness.send_key(KeyCode::PageDown);
    }
    let scrolled_output = harness.render_to_string();

    // WHEN: User presses Page Up
    harness.send_key(KeyCode::PageUp);

    // THEN: Viewport scrolls up by approximately viewport height
    let paged_up_output = harness.render_to_string();

    assert_ne!(
        scrolled_output, paged_up_output,
        "Pressing Page Up should scroll viewport up by viewport height"
    );

    // Verify with snapshot
    insta::assert_snapshot!("page_up", paged_up_output);
}

#[test]
fn page_up_multiple_times_continues_scrolling() {
    // GIVEN: Viewer scrolled down
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(SCROLL_FIXTURE, 80, 24)
        .expect("Should load fixture");

    harness.render_to_string();

    // Scroll down several pages (use 3x to stay within scrollable range)
    for _ in 0..3 {
        harness.send_key(KeyCode::PageDown);
    }

    // Press Page Up once
    harness.send_key(KeyCode::PageUp);
    let first_page_up_output = harness.render_to_string();

    // WHEN: User presses Page Up again
    harness.send_key(KeyCode::PageUp);

    // THEN: Viewport scrolls up further
    let second_page_up_output = harness.render_to_string();

    assert_ne!(
        first_page_up_output, second_page_up_output,
        "Multiple Page Up presses should continue scrolling up"
    );

    // Verify with snapshot
    insta::assert_snapshot!("page_up_2x", second_page_up_output);
}

// ===== Scroll Overshoot Bug Reproduction =====

/// Bug reproduction: cclv-5ur.77 - Scroll offset overshoot causes k to be absorbed
///
/// EXPECTED: After scrolling to bottom and overshooting with j, k should scroll up.
/// After scrolling back down with j, k should STILL scroll up immediately.
///
/// ACTUAL (bug): After G → j×5 → k (works) → j×5, pressing k is ABSORBED.
/// The internal scroll offset gets corrupted when scrolling up from overshoot
/// then scrolling back down past bottom. Multiple k presses needed before
/// the display changes.
///
/// Steps to reproduce manually:
/// 1. cargo run -- tests/fixtures/scroll_overshoot_repro.jsonl
/// 2. G (go to bottom) - shows Entry 80+
/// 3. j×5 (overshoot at bottom) - still shows Entry 80+
/// 4. k (works! scrolls up) - shows Entry 79+
/// 5. j×5 (back to bottom + overshoot) - shows Entry 80+
/// 6. k (FIXED: now works!) - Shows Entry 79+, k scrolls up correctly
#[test]
fn bug_scroll_overshoot_k_absorbed() {
    const FIXTURE: &str = "tests/fixtures/scroll_overshoot_repro.jsonl";

    let mut harness =
        AcceptanceTestHarness::from_fixture_with_size(FIXTURE, 80, 24).expect("Should load fixture");

    // Step 1: G (go to bottom)
    harness.send_key_with_mods(KeyCode::Char('G'), crossterm::event::KeyModifiers::SHIFT);
    let at_bottom = harness.render_to_string();
    assert!(
        at_bottom.contains("MARKER_ENTRY_LAST"),
        "After G, should be at bottom showing MARKER_ENTRY_LAST"
    );

    // Step 2: j×5 (overshoot at bottom - display clamped)
    for _ in 0..5 {
        harness.send_key(KeyCode::Char('j'));
    }
    let after_j5_first = harness.render_to_string();
    assert_eq!(
        at_bottom, after_j5_first,
        "j×5 at bottom should be no-op (display clamped)"
    );

    // Step 3: k (should work - scroll up from overshoot)
    harness.send_key(KeyCode::Char('k'));
    let after_k_first = harness.render_to_string();
    assert_ne!(
        after_j5_first, after_k_first,
        "First k after overshoot should scroll up (screen should change)"
    );

    // Step 4: j×5 (scroll back to bottom + overshoot again)
    for _ in 0..5 {
        harness.send_key(KeyCode::Char('j'));
    }
    let after_j5_second = harness.render_to_string();
    // Should be back at bottom
    assert!(
        after_j5_second.contains("MARKER_ENTRY_LAST"),
        "After j×5, should be back at bottom"
    );

    // Step 5: k (BUG - this k is absorbed!)
    harness.send_key(KeyCode::Char('k'));
    let after_k_second = harness.render_to_string();

    // Capture states for debugging (before assertion so they're always created)
    insta::assert_snapshot!("scroll_overshoot_step4_back_at_bottom", after_j5_second);
    insta::assert_snapshot!("scroll_overshoot_step5_after_second_k", after_k_second);

    // KEY ASSERTION: screen must have changed after k
    // If bug exists: after_k_second == after_j5_second (k was absorbed)
    // If bug fixed: after_k_second != after_j5_second (scrolled up)
    assert_ne!(
        after_j5_second, after_k_second,
        "BUG cclv-5ur.77: k was absorbed after G→j×5→k→j×5 sequence.\n\
         Expected: k should scroll up (screen changes).\n\
         Actual: k absorbed, screen unchanged."
    );
}

// ===== Scroll Roundtrip Tests =====

#[test]
fn scroll_down_then_up_returns_to_original_position() {
    // GIVEN: Viewer at top
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(SCROLL_FIXTURE, 80, 24)
        .expect("Should load fixture");

    harness.render_to_string();
    let initial_output = harness.render_to_string();

    // WHEN: User scrolls down 10 lines then up 10 lines
    for _ in 0..10 {
        harness.send_key(KeyCode::Char('j'));
    }
    for _ in 0..10 {
        harness.send_key(KeyCode::Char('k'));
    }

    // THEN: Viewport shows original content
    let returned_output = harness.render_to_string();

    assert_eq!(
        initial_output, returned_output,
        "Scrolling down N lines then up N lines should return to original viewport"
    );
}

#[test]
fn jump_to_bottom_then_top_shows_original_content() {
    // GIVEN: Viewer at top
    let mut harness = AcceptanceTestHarness::from_fixture_with_size(SCROLL_FIXTURE, 80, 24)
        .expect("Should load fixture");

    harness.render_to_string();
    let initial_output = harness.render_to_string();

    // WHEN: User jumps to bottom then back to top
    harness.send_key_with_mods(KeyCode::Char('G'), crossterm::event::KeyModifiers::SHIFT);
    harness.send_key(KeyCode::Char('g'));

    // THEN: Viewport shows original content
    let returned_output = harness.render_to_string();

    assert_eq!(
        initial_output, returned_output,
        "Jumping to bottom then top should return to original viewport"
    );
}
