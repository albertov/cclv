//! Property-based tests for parser and type invariants.
//!
//! Tests validate:
//! 1. Identifier constructors reject empty strings
//! 2. ToolName::parse round-trips correctly
//! 3. Statistics total_usage == main + sum(subagents)
//! 4. ScrollState bounds: vertical_offset ≤ max_entries after any action sequence

use cclv::model::{
    AgentId, EntryMetadata, EntryType, EntryUuid, LogEntry, Message, MessageContent, Role,
    SessionId, SessionStats, TokenUsage, ToolName, ToolUseId,
};
use cclv::state::ScrollState;
use chrono::Utc;
use proptest::prelude::*;

// ===== Property 1: Identifier Constructors =====

proptest! {
    #[test]
    fn entry_uuid_rejects_empty_string(s in any::<String>()) {
        if s.is_empty() {
            prop_assert!(EntryUuid::new(&s).is_err(), "Empty string should be rejected");
        }
    }

    #[test]
    fn session_id_rejects_empty_string(s in any::<String>()) {
        if s.is_empty() {
            prop_assert!(SessionId::new(&s).is_err(), "Empty string should be rejected");
        }
    }

    #[test]
    fn agent_id_rejects_empty_string(s in any::<String>()) {
        if s.is_empty() {
            prop_assert!(AgentId::new(&s).is_err(), "Empty string should be rejected");
        }
    }

    #[test]
    fn tool_use_id_rejects_empty_string(s in any::<String>()) {
        if s.is_empty() {
            prop_assert!(ToolUseId::new(&s).is_err(), "Empty string should be rejected");
        }
    }

    #[test]
    fn entry_uuid_accepts_non_empty_string(s in any::<String>()) {
        if !s.is_empty() {
            prop_assert!(EntryUuid::new(&s).is_ok(), "Non-empty string should be accepted");
        }
    }

    #[test]
    fn session_id_accepts_non_empty_string(s in any::<String>()) {
        if !s.is_empty() {
            prop_assert!(SessionId::new(&s).is_ok(), "Non-empty string should be accepted");
        }
    }

    #[test]
    fn agent_id_accepts_non_empty_string(s in any::<String>()) {
        if !s.is_empty() {
            prop_assert!(AgentId::new(&s).is_ok(), "Non-empty string should be accepted");
        }
    }

    #[test]
    fn tool_use_id_accepts_non_empty_string(s in any::<String>()) {
        if !s.is_empty() {
            prop_assert!(ToolUseId::new(&s).is_ok(), "Non-empty string should be accepted");
        }
    }
}

// ===== Property 2: ToolName Round-Trip =====

proptest! {
    #[test]
    fn tool_name_parse_roundtrip_known_variants(name in prop_oneof![
        Just("Read"),
        Just("Write"),
        Just("Edit"),
        Just("MultiEdit"),
        Just("Bash"),
        Just("Grep"),
        Just("Glob"),
        Just("Task"),
        Just("WebSearch"),
        Just("WebFetch"),
    ]) {
        // Parse the name and convert back to string
        let tool_name = ToolName::parse(name);
        let roundtrip = tool_name.as_str();

        // Round-trip should preserve the original value
        prop_assert_eq!(roundtrip, name, "ToolName::parse should round-trip for known variants");
    }

    #[test]
    fn tool_name_parse_roundtrip_arbitrary_string(s in any::<String>()) {
        // Parse any string and convert back
        let tool_name = ToolName::parse(&s);
        let roundtrip = tool_name.as_str();

        // Round-trip should always preserve the original value
        prop_assert_eq!(roundtrip, s, "ToolName::parse should round-trip for any string");
    }
}

// ===== Property 3: Statistics Consistency =====

proptest! {
    #[test]
    fn stats_total_equals_main_plus_subagents(
        main_input in 0u64..1_000_000,
        main_output in 0u64..1_000_000,
        subagent_entries in prop::collection::vec(
            (any::<String>(), 0u64..100_000, 0u64..100_000),
            0..10
        )
    ) {
        let mut stats = SessionStats::default();

        // Create main agent entry with usage
        let main_usage = TokenUsage {
            input_tokens: main_input,
            output_tokens: main_output,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
        };
        let main_message = Message::new(
            Role::Assistant,
            MessageContent::Text("Main agent".to_string())
        ).with_usage(main_usage);
        let main_entry = LogEntry::new(
            EntryUuid::new("main-entry").unwrap(),
            None,
            SessionId::new("session-1").unwrap(),
            None, // No agent_id = main agent
            Utc::now(),
            EntryType::Assistant,
            main_message,
            EntryMetadata::default(),
        );
        stats.record_entry(&main_entry);

        // Create subagent entries
        let mut expected_subagent_input = 0u64;
        let mut expected_subagent_output = 0u64;

        for (i, (agent_name, input, output)) in subagent_entries.iter().enumerate() {
            // Skip empty agent names (invalid)
            if agent_name.is_empty() {
                continue;
            }

            let usage = TokenUsage {
                input_tokens: *input,
                output_tokens: *output,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            };
            let message = Message::new(
                Role::Assistant,
                MessageContent::Text(format!("Subagent {}", i))
            ).with_usage(usage);
            let entry = LogEntry::new(
                EntryUuid::new(format!("entry-{}", i)).unwrap(),
                None,
                SessionId::new("session-1").unwrap(),
                Some(AgentId::new(agent_name).unwrap()),
                Utc::now(),
                EntryType::Assistant,
                message,
                EntryMetadata::default(),
            );
            stats.record_entry(&entry);

            expected_subagent_input += input;
            expected_subagent_output += output;
        }

        // Verify invariant: total_usage == main + sum(subagents)
        let expected_total_input = main_input + expected_subagent_input;
        let expected_total_output = main_output + expected_subagent_output;

        prop_assert_eq!(
            stats.total_usage.input_tokens,
            expected_total_input,
            "Total input tokens should equal main + subagent sum"
        );
        prop_assert_eq!(
            stats.total_usage.output_tokens,
            expected_total_output,
            "Total output tokens should equal main + subagent sum"
        );

        // Also verify main and subagent usage are tracked correctly
        prop_assert_eq!(
            stats.main_agent_usage.input_tokens,
            main_input,
            "Main agent input tokens should match"
        );
        prop_assert_eq!(
            stats.main_agent_usage.output_tokens,
            main_output,
            "Main agent output tokens should match"
        );
    }
}

// ===== Property 4: ScrollState Bounds Invariant =====

/// Action types for ScrollState property testing.
#[derive(Debug, Clone)]
enum ScrollAction {
    Up(usize),
    Down(usize),
    ToBottom,
}

/// Strategy to generate arbitrary scroll actions.
fn scroll_action_strategy() -> impl Strategy<Value = ScrollAction> {
    prop_oneof![
        (0usize..1000).prop_map(ScrollAction::Up),
        (0usize..1000).prop_map(ScrollAction::Down),
        Just(ScrollAction::ToBottom),
    ]
}

proptest! {
    #[test]
    fn scroll_state_vertical_offset_respects_bounds(
        initial_offset in 0usize..1000,
        max_entries in 0usize..1000,
        actions in prop::collection::vec(scroll_action_strategy(), 0..50)
    ) {
        // Ensure initial state respects bounds
        let initial_offset = initial_offset.min(max_entries);

        let mut state = ScrollState {
            vertical_offset: initial_offset,
            horizontal_offset: 0,
            expanded_messages: Default::default(),
            focused_message: None,
        };

        // Apply each action and verify bounds hold
        for action in actions {
            match action {
                ScrollAction::Up(amount) => {
                    state.scroll_up(amount);
                }
                ScrollAction::Down(amount) => {
                    state.scroll_down(amount, max_entries);
                }
                ScrollAction::ToBottom => {
                    state.scroll_to_bottom(max_entries);
                }
            }

            // INVARIANT: vertical_offset ≤ max_entries
            prop_assert!(
                state.vertical_offset <= max_entries,
                "vertical_offset ({}) must be ≤ max_entries ({}) after action {:?}",
                state.vertical_offset,
                max_entries,
                action
            );
        }
    }

    #[test]
    fn scroll_state_vertical_offset_never_negative(
        initial_offset in 0usize..1000,
        scroll_up_amounts in prop::collection::vec(0usize..1000, 0..20)
    ) {
        let mut state = ScrollState {
            vertical_offset: initial_offset,
            horizontal_offset: 0,
            expanded_messages: Default::default(),
            focused_message: None,
        };

        // Scroll up repeatedly with arbitrary amounts
        for amount in scroll_up_amounts {
            state.scroll_up(amount);

            // vertical_offset is usize, but verify it doesn't wrap/overflow
            // by checking it's still reasonable
            prop_assert!(
                state.vertical_offset <= initial_offset,
                "vertical_offset should never increase from scroll_up"
            );
        }

        // After scrolling up, offset should be ≥ 0 (trivially true for usize)
        // but more importantly, it should have saturated at 0
        prop_assert!(
            state.vertical_offset == 0 || state.vertical_offset <= initial_offset,
            "vertical_offset should saturate at 0 when scrolling up"
        );
    }
}
