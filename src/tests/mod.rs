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
