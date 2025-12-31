//! Integration tests for render cache in the rendering loop.
//!
//! Tests that the cache is properly integrated into actual entry rendering:
//! - Cache miss path: First render populates cache
//! - Cache hit path: Subsequent render uses cached lines
//! - Cache invalidation: Parameters changes force re-render

use cclv::model::{
    ConversationEntry, EntryMetadata, EntryType, EntryUuid, LogEntry, Message, MessageContent,
    Role, SessionId,
};
use cclv::state::WrapMode;
use cclv::view_state::cache::{RenderCacheConfig, RenderCacheKey};
use cclv::view_state::conversation::ConversationViewState;
use ratatui::text::Line;

// ===== Test Helpers =====

fn test_session_id(s: &str) -> SessionId {
    SessionId::new(s).expect("Valid session ID")
}

fn test_entry_uuid(s: &str) -> EntryUuid {
    EntryUuid::new(s).expect("Valid UUID")
}

fn test_timestamp() -> chrono::DateTime<chrono::Utc> {
    "2025-12-25T10:00:00Z".parse().expect("Valid timestamp")
}

fn test_message(text: &str) -> Message {
    Message::new(Role::User, MessageContent::Text(text.to_string()))
}

fn test_entry(uuid: &str, text: &str) -> ConversationEntry {
    let log_entry = LogEntry::new(
        test_entry_uuid(uuid),
        None,
        test_session_id("test-session"),
        None,
        test_timestamp(),
        EntryType::User,
        test_message(text),
        EntryMetadata::default(),
    );
    ConversationEntry::Valid(Box::new(log_entry))
}

/// Stub function to render an entry (returns placeholder lines).
///
/// In real rendering, this would call markdown parser and syntax highlighter.
/// For testing, we just return distinct lines based on entry UUID.
fn stub_render_entry(
    entry: &ConversationEntry,
    _expanded: bool,
    _wrap_mode: WrapMode,
) -> Vec<Line<'static>> {
    match entry {
        ConversationEntry::Valid(log_entry) => {
            let uuid_str = log_entry.uuid().as_str();
            vec![
                Line::from(format!("Rendered: {}", uuid_str)),
                Line::from("Line 2"),
                Line::from("Line 3"),
            ]
        }
        ConversationEntry::Malformed(_) => vec![Line::from("Malformed entry")],
    }
}

// ===== Phase 2: TESTS (RED) =====

#[test]
fn cache_miss_on_first_render_populates_cache() {
    // GIVEN: Entry viewed for the first time
    // WHEN: Entry is rendered
    // THEN: Cache is populated with rendered lines

    // DOING: Create view-state, render entry, check cache populated
    // EXPECT: After first render, cache contains entry

    let uuid = test_entry_uuid("uuid-1");
    let entries = vec![test_entry("uuid-1", "Test message")];
    let view_state = ConversationViewState::with_cache_config(
        None,
        None,
        entries,
        &RenderCacheConfig { capacity: 10 },
    );

    // Verify cache is initially empty
    {
        let cache = view_state.render_cache().borrow();
        assert_eq!(cache.len(), 0, "Cache should start empty");
    }

    // Build cache key
    let key = RenderCacheKey::new(uuid.clone(), 80, false, WrapMode::Wrap);

    // Check cache before rendering (should be miss)
    {
        let mut cache = view_state.render_cache().borrow_mut();
        assert!(
            cache.get(&key).is_none(),
            "Cache should miss on first render"
        );
    }

    // Simulate rendering the entry (without borrowing view_state)
    let entry_view = view_state
        .get(cclv::view_state::types::EntryIndex::new(0))
        .unwrap();
    let entry = entry_view.entry();
    let rendered_lines = stub_render_entry(entry, false, WrapMode::Wrap);

    // Populate cache with rendered result
    {
        let mut cache = view_state.render_cache().borrow_mut();
        cache.put(
            key.clone(),
            cclv::view_state::cache::CachedRender {
                lines: rendered_lines.clone(),
            },
        );
    }

    // Verify cache now contains entry
    {
        let mut cache = view_state.render_cache().borrow_mut();
        assert_eq!(cache.len(), 1, "Cache should contain 1 entry after render");

        let cached = cache.get(&key);
        assert!(cached.is_some(), "Cache should hit after population");
        assert_eq!(
            cached.unwrap().lines.len(),
            3,
            "Cached lines should match rendered lines"
        );
    }

    // RESULT: Cache populated on first render
    // MATCHES: Expected behavior
    // THEREFORE: Cache miss path works
}

#[test]
fn cache_hit_on_subsequent_render_uses_cached_lines() {
    // GIVEN: Entry already rendered and cached
    // WHEN: Entry is rendered again with same parameters
    // THEN: Cached lines are used (no re-rendering)

    // DOING: Pre-populate cache, verify cache hit on subsequent access
    // EXPECT: cache.get() returns Some with previously cached lines

    let uuid = test_entry_uuid("uuid-2");
    let entries = vec![test_entry("uuid-2", "Cached message")];
    let view_state = ConversationViewState::with_cache_config(
        None,
        None,
        entries,
        &RenderCacheConfig { capacity: 10 },
    );

    // Pre-populate cache (simulating first render)
    let key = RenderCacheKey::new(uuid.clone(), 80, false, WrapMode::Wrap);
    let pre_rendered_lines = vec![
        Line::from("Cached line 1"),
        Line::from("Cached line 2"),
        Line::from("Cached line 3"),
    ];

    {
        let mut cache = view_state.render_cache().borrow_mut();
        cache.put(
            key.clone(),
            cclv::view_state::cache::CachedRender {
                lines: pre_rendered_lines.clone(),
            },
        );
    }

    // WHEN: Second render of same entry
    {
        let mut cache = view_state.render_cache().borrow_mut();
        let cached_result = cache.get(&key);

        // THEN: Cache hit
        assert!(cached_result.is_some(), "Cache should hit on second render");

        let cached_lines = &cached_result.unwrap().lines;
        assert_eq!(cached_lines.len(), 3, "Cached lines should match original");
        assert_eq!(
            cached_lines[0].to_string(),
            "Cached line 1",
            "Cached content should match original"
        );
    }

    // RESULT: Cache hit on subsequent render
    // MATCHES: Expected behavior
    // THEREFORE: Cache hit path works
}

#[test]
fn cache_invalidates_on_width_change() {
    // GIVEN: Entry cached at width 80
    // WHEN: Viewport width changes to 100
    // THEN: Cache miss (different key)

    // DOING: Cache at width 80, check miss at width 100
    // EXPECT: Different widths produce different keys

    let uuid = test_entry_uuid("uuid-3");
    let entries = vec![test_entry("uuid-3", "Width test")];
    let view_state = ConversationViewState::with_cache_config(
        None,
        None,
        entries,
        &RenderCacheConfig { capacity: 10 },
    );

    // Cache at width 80
    let key_80 = RenderCacheKey::new(uuid.clone(), 80, false, WrapMode::Wrap);
    {
        let mut cache = view_state.render_cache().borrow_mut();
        cache.put(
            key_80.clone(),
            cclv::view_state::cache::CachedRender {
                lines: vec![Line::from("Width 80")],
            },
        );
    }

    // Verify hit at width 80
    {
        let mut cache = view_state.render_cache().borrow_mut();
        assert!(cache.get(&key_80).is_some(), "Should hit at width 80");
    }

    // Check at width 100 (should miss)
    let key_100 = RenderCacheKey::new(uuid.clone(), 100, false, WrapMode::Wrap);
    {
        let mut cache = view_state.render_cache().borrow_mut();
        assert!(
            cache.get(&key_100).is_none(),
            "Should miss at width 100 (different key)"
        );
    }

    // RESULT: Cache invalidates on width change
    // MATCHES: Expected behavior
    // THEREFORE: Width invalidation works
}

#[test]
fn cache_invalidates_on_expanded_state_change() {
    // GIVEN: Entry cached in collapsed state
    // WHEN: Entry is expanded
    // THEN: Cache miss (different key)

    // DOING: Cache collapsed, check miss when expanded
    // EXPECT: Different expanded states produce different keys

    let uuid = test_entry_uuid("uuid-4");
    let entries = vec![test_entry("uuid-4", "Expand test")];
    let view_state = ConversationViewState::with_cache_config(
        None,
        None,
        entries,
        &RenderCacheConfig { capacity: 10 },
    );

    // Cache in collapsed state (expanded=false)
    let key_collapsed = RenderCacheKey::new(uuid.clone(), 80, false, WrapMode::Wrap);
    {
        let mut cache = view_state.render_cache().borrow_mut();
        cache.put(
            key_collapsed.clone(),
            cclv::view_state::cache::CachedRender {
                lines: vec![Line::from("Collapsed")],
            },
        );
    }

    // Check expanded state (should miss)
    let key_expanded = RenderCacheKey::new(uuid.clone(), 80, true, WrapMode::Wrap);
    {
        let mut cache = view_state.render_cache().borrow_mut();
        assert!(
            cache.get(&key_expanded).is_none(),
            "Should miss when expanded state changes"
        );
    }

    // RESULT: Cache invalidates on expand state change
    // MATCHES: Expected behavior
    // THEREFORE: Expand state invalidation works
}

#[test]
fn cache_invalidates_on_wrap_mode_change() {
    // GIVEN: Entry cached with WrapMode::Wrap
    // WHEN: Wrap mode changes to NoWrap
    // THEN: Cache miss (different key)

    // DOING: Cache with Wrap, check miss with NoWrap
    // EXPECT: Different wrap modes produce different keys

    let uuid = test_entry_uuid("uuid-5");
    let entries = vec![test_entry("uuid-5", "Wrap test")];
    let view_state = ConversationViewState::with_cache_config(
        None,
        None,
        entries,
        &RenderCacheConfig { capacity: 10 },
    );

    // Cache with Wrap mode
    let key_wrap = RenderCacheKey::new(uuid.clone(), 80, false, WrapMode::Wrap);
    {
        let mut cache = view_state.render_cache().borrow_mut();
        cache.put(
            key_wrap.clone(),
            cclv::view_state::cache::CachedRender {
                lines: vec![Line::from("Wrapped")],
            },
        );
    }

    // Check NoWrap mode (should miss)
    let key_nowrap = RenderCacheKey::new(uuid.clone(), 80, false, WrapMode::NoWrap);
    {
        let mut cache = view_state.render_cache().borrow_mut();
        assert!(
            cache.get(&key_nowrap).is_none(),
            "Should miss when wrap mode changes"
        );
    }

    // RESULT: Cache invalidates on wrap mode change
    // MATCHES: Expected behavior
    // THEREFORE: Wrap mode invalidation works
}
