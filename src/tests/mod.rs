//! Internal test modules - whitebox tests with crate access
//!
//! This module contains tests that require internal access to crate types.
//! Tests here can access private items and implementation details for
//! comprehensive validation of internal invariants and edge cases.

mod parse_real_logs;
mod tui_integration;
mod property_tests;
mod key_action_tests;
mod view_state_invariants;
mod view_state_us2_expand_collapse;
mod view_state_us3_mouse_hit_testing;

// Harness-based acceptance tests
mod harness_test;
mod acceptance_us1;
mod acceptance_us2;
mod acceptance_us3;
mod acceptance_us4;
mod acceptance_us5;
mod acceptance_scroll;
mod crash_regression;
mod help_overlay_tests;

// Whitebox tests with internal access
mod view_snapshots;
mod event_driven_rendering;
mod scroll_properties;
mod tool_block_nowrap_default;
mod stats_multi_scope_tests;
