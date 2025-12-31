# Implementation Plan: Claude Code Log Viewer TUI

**Branch**: `001-claude-code-log-viewer` | **Date**: 2025-12-25 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-claude-code-log-viewer/spec.md`

## Summary

Build a high-performance TUI application to view Claude Code JSONL log files in real-time or for post-mortem analysis. The application displays main agent and subagent conversations in split panes with tabs, supports live tailing, search, statistics, markdown rendering with syntax highlighting, and full keyboard navigation. Target: 60fps rendering, <500ms latency.

## Technical Context

**Language/Version**: Rust stable (latest, currently 1.83+)
**Primary Dependencies**: ratatui (TUI framework), crossterm (terminal backend), serde_json (JSONL parsing), notify (file watching), syntect (syntax highlighting), toml (config parsing), dirs (XDG config paths)
**Build System**: Nix flake with naersk for reproducible Rust builds
**Storage**: N/A - loads file into memory, no persistence
**Testing**: cargo test, proptest (property-based), insta (snapshot testing)
**Target Platform**: Linux/macOS terminals with 256-color or true-color support
**Project Type**: Single CLI application at repository root
**Performance Goals**: 60fps UI, <500ms write-to-display latency, <1s search in 50MB file, <1s startup for <10MB files
**Constraints**: Virtualized rendering for large logs (v1 loads entire file into memory)
**Scale/Scope**: Single-user CLI tool, handles logs up to 1GB+

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Design Principles (APPLICABLE)

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Type-Driven Design | WILL COMPLY | Domain types for LogEntry, Message, Agent, ContentBlock, KeyAction with smart constructors |
| II. Deep Module Architecture | WILL COMPLY | Hide JSONL parsing, rendering, file tailing behind simple interfaces |
| III. Denotational Semantics | WILL COMPLY | Define state transition semantics for UI (scroll, focus, search) |
| IV. Total Functions & Railway Programming | WILL COMPLY | Result types for parsing, file operations; thiserror for typed errors |
| V. Pure Core, Impure Shell | WILL COMPLY | Pure: state transitions, search logic. Impure: file I/O, terminal rendering |
| VI. Property-Based Testing | WILL COMPLY | proptest for parsing round-trips, search invariants, state machine properties |

### Domain Principles (NOT APPLICABLE)

| Principle | Status | Notes |
|-----------|--------|-------|
| VII. GPU-First Design | N/A | TUI application, no GPU |
| VIII. no_std Compatibility | N/A | Uses std freely |
| IX. Physical Correctness | N/A | No physics simulation |
| X. Const-Driven Initialization | N/A | Runtime configuration only |

### Quality Gates (Pre-Implementation)

- [x] **Types designed first**: Will design in data-model.md before implementation
- [x] **No illegal states**: Sum types for AgentKind, MessageType, FocusPane, ScrollState
- [x] **Smart constructors**: For validated types (TokenCount, SearchQuery, etc.)
- [x] **Total functions**: No panics in public API
- [x] **Pure domain logic**: State transitions testable without TUI
- [x] **Property tests**: Will add for parser and state machine
- [ ] **GPU compatible**: N/A
- [ ] **no_std compliant**: N/A
- [ ] **Build passes**: Pending implementation
- [ ] **Tests pass**: Pending implementation
- [ ] **Linting clean**: Pending implementation

**Gate Status**: PASS (design principles satisfied, domain principles not applicable)

## Project Structure

### Documentation (this feature)

```text
specs/001-claude-code-log-viewer/
├── plan.md              # This file
├── research.md          # Phase 0: Technology decisions
├── data-model.md        # Phase 1: Type definitions
├── quickstart.md        # Phase 1: Getting started guide
├── contracts/           # Phase 1: CLI interface contract
│   └── cli.md           # Command-line interface specification
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
./
├── flake.nix            # Nix flake: devShell, package build with naersk
├── flake.lock           # Locked dependencies
├── nix/
│   ├── devshell.nix     # Development shell with Rust toolchain
│   └── treefmt.nix      # Code formatting configuration
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, CLI parsing
│   ├── lib.rs               # Public API
│   ├── model/               # Domain types (pure)
│   │   ├── mod.rs
│   │   ├── log_entry.rs     # LogEntry, Message, ContentBlock
│   │   ├── session.rs       # Session, Agent hierarchy
│   │   ├── stats.rs         # TokenStats, ToolStats
│   │   └── key_action.rs    # KeyAction enum, KeyBinding
│   ├── parser/              # JSONL parsing (pure)
│   │   ├── mod.rs
│   │   └── content_block.rs # ContentBlock variants
│   ├── state/               # UI state machine (pure)
│   │   ├── mod.rs
│   │   ├── app_state.rs     # AppState, transitions
│   │   ├── scroll.rs        # ScrollState per pane
│   │   └── search.rs        # SearchState
│   ├── source/              # Log input sources (impure shell)
│   │   ├── mod.rs
│   │   ├── file.rs          # File source with tailing
│   │   └── stdin.rs         # Stdin source
│   ├── view/                # TUI rendering (impure shell)
│   │   ├── mod.rs
│   │   ├── layout.rs        # Split pane layout
│   │   ├── message.rs       # Message rendering with markdown
│   │   ├── tabs.rs          # Subagent tab bar
│   │   └── stats.rs         # Statistics panel
│   └── config/              # Configuration (unified config.toml)
│       ├── mod.rs           # AppConfig with hardcoded defaults + optional TOML merge
│       └── keybindings.rs   # KeyAction to key mappings
└── tests/
    ├── fixtures/               # Test data (extracted from sample logs)
    │   ├── minimal_session.jsonl
    │   ├── with_subagents.jsonl
    │   ├── tool_calls.jsonl
    │   ├── malformed_lines.jsonl
    │   └── large_message.jsonl
    ├── integration/
    │   └── parse_real_logs.rs
    └── property/
        └── parser_roundtrip.rs
```

**Structure Decision**: Single crate (`cclv`) at repository root. Separates pure domain logic (`model/`, `parser/`, `state/`) from impure shell (`source/`, `view/`).

**Shared Rendering Architecture**: The main agent pane and subagent pane MUST use identical rendering code. Both render `AgentConversation` (same type for main and subagents). The `src/view/message.rs` module provides a single `ConversationView` widget that:
- Accepts `&AgentConversation` and `&ScrollState` as input
- Renders messages with markdown, syntax highlighting, expand/collapse
- Handles virtualization for performance
- Is instantiated twice in the layout: once for main pane, once for active subagent tab

This ensures visual consistency, reduces code duplication, and means features like search highlighting, expand/collapse, and styling work identically in both panes. The only difference is the layout context (left pane vs tabbed right pane).

### Sample Session Logs

Real Claude Code session logs are available for exploratory testing and fixture extraction:

```
~/*.jsonl
├── 007-proptest-terrain.log.jsonl   (61M)  # Large session with extensive tool usage
├── investigation-log.jsonl          (19M)  # Investigation/debugging session
├── session-log.jsonl                (27M)  # General session
├── session-log.003.jsonl            (13M)  # Numbered sessions
├── session-log.004.jsonl            (53M)  # Large multi-subagent session
└── ... (additional sessions)
```

**⚠️ CRITICAL: Test Fixture Policy**

These files are for **exploratory purposes and fixture extraction ONLY**:

- **NEVER** reference `~/*.jsonl` files directly in tests or source code
- **NEVER** use absolute paths to home directory in the codebase
- **ALWAYS** copy relevant JSONL lines to `tests/fixtures/` when creating test cases
- **ALWAYS** use relative paths within the project for test fixtures

```text
tests/
└── fixtures/
    ├── minimal_session.jsonl      # Minimal valid session (extracted lines)
    ├── with_subagents.jsonl       # Session with subagent spawns
    ├── tool_calls.jsonl           # Various tool call examples
    ├── malformed_lines.jsonl      # Invalid JSON for error handling tests
    └── large_message.jsonl        # Long message for collapse testing
```

**Rationale**: Tests must be self-contained and reproducible. External file dependencies break CI/CD, make tests non-portable, and create implicit coupling to developer environments.

## Complexity Tracking

> No Constitution violations requiring justification.

| Aspect | Decision | Rationale |
|--------|----------|-----------|
| Project structure | Single crate at root | Simple standalone TUI application |
| Async vs sync | Sync with polling | TUI event loops don't benefit from async; simpler model |
| Full markdown parser vs subset | Subset parser | Only need headings, bold, italic, code blocks, lists for log viewing |
| Pane rendering | Shared `ConversationView` widget | Main and subagent panes render same `AgentConversation` type; single implementation ensures consistency and DRY |

---

## Constitution Check (Post-Design)

*Re-evaluation after Phase 1 design completion.*

### Design Principles Verification

| Principle | Status | Evidence |
|-----------|--------|----------|
| I. Type-Driven Design | ✅ COMPLIANT | data-model.md: Newtypes for all IDs (EntryUuid, SessionId, AgentId), sum types for MessageContent, EntryType, FocusPane. Smart constructors only. |
| II. Deep Module Architecture | ✅ COMPLIANT | Minimal exports: `Session::add_entry()`, `LogEntry::parse()`. Implementation hidden in modules. |
| III. Denotational Semantics | ✅ COMPLIANT | data-model.md defines clear semantics: ScrollState transitions, SearchState machine states. |
| IV. Total Functions & Railway Programming | ✅ COMPLIANT | All parse functions return Result. Error types defined per module with thiserror. No panics in public API. |
| V. Pure Core, Impure Shell | ✅ COMPLIANT | Pure: model/, parser/, state/ modules. Impure: source/, view/ modules. |
| VI. Property-Based Testing | ✅ PLANNED | data-model.md: Invariants documented (scroll bounds, statistics consistency, search match validity). |

### Quality Gates (Post-Design)

- [x] **Types designed first**: Complete in data-model.md
- [x] **No illegal states**: Sum types enforce valid states throughout
- [x] **Smart constructors**: All identifiers use smart constructors (never export raw)
- [x] **Total functions**: Error types comprehensive, no unwrap in public API
- [x] **Pure domain logic**: Clean separation in module structure
- [x] **Property tests**: Invariants documented, ready for implementation
- [ ] **Build passes**: Pending implementation
- [ ] **Tests pass**: Pending implementation
- [ ] **Linting clean**: Pending implementation

**Post-Design Gate Status**: ✅ PASS

---

## Phase 0: Nix Development Environment

**Purpose**: Establish reproducible development environment before implementation begins.

### Nix Flake Design

The project uses a Nix flake with the following structure:

```nix
# flake.nix - Claude Code Log Viewer
{
  description = "Claude Code Log Viewer - TUI for viewing Claude Code JSONL logs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-25.11";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];

      perSystem = { config, self', inputs', system, pkgs, lib, ... }:
        let
          # Rust toolchain from overlay (with musl targets for static builds)
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
            targets = [ "x86_64-unknown-linux-musl" "aarch64-unknown-linux-musl" ];
          };

          # naersk configured with our toolchain
          naersk' = pkgs.callPackage inputs.naersk {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };

          # Static build configuration for Linux
          isLinux = pkgs.stdenv.isLinux;
          staticTarget = if pkgs.stdenv.hostPlatform.isx86_64
                         then "x86_64-unknown-linux-musl"
                         else "aarch64-unknown-linux-musl";
        in {
          # Default package (dynamic linking)
          packages.default = naersk'.buildPackage {
            src = ./.;
            doCheck = true;
          };

          # Static package for Linux (fully static, no glibc dependency)
          packages.static = lib.mkIf isLinux (naersk'.buildPackage {
            src = ./.;
            doCheck = true;
            CARGO_BUILD_TARGET = staticTarget;
            CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
            nativeBuildInputs = [ pkgs.pkgsStatic.stdenv.cc ];
          });

          # Development shell
          devShells.default = pkgs.mkShell {
            inputsFrom = [ self'.packages.default ];
            packages = with pkgs; [
              rustToolchain
              cargo-watch
              cargo-edit
              cargo-outdated
            ];
          };

          # Formatter (nix fmt)
          formatter = treefmtEval.config.build.wrapper;
        };
    };
}
```

### Development Shell Contents

The devShell provides:

| Tool | Purpose |
|------|---------|
| `rust-bin.stable.latest` | Rust stable toolchain with rust-analyzer |
| `cargo-watch` | Auto-rebuild on file changes |
| `cargo-edit` | Cargo add/rm/upgrade commands |
| `cargo-outdated` | Check for outdated dependencies |

### Formatting Configuration

```nix
# nix/treefmt.nix
{ pkgs, ... }: {
  projectRootFile = "flake.nix";
  programs = {
    nixfmt.enable = true;      # Nix formatting
    rustfmt.enable = true;     # Rust formatting
    taplo.enable = true;       # TOML formatting (Cargo.toml)
  };
}
```

### Usage

```bash
# Enter development shell
nix develop

# Build the package (dynamic linking)
nix build

# Build static binary (Linux only, no glibc dependency)
nix build .#static

# Run the application
nix run . -- ~/.claude/projects/.../session.jsonl

# Format all code
nix fmt

# Check formatting
nix flake check

# Verify static binary has no dynamic dependencies
ldd result/bin/cclv  # Should show "not a dynamic executable"
```

### Static Binary Distribution

The flake provides statically compiled executables for Linux:

| Architecture | Target | Command |
|--------------|--------|---------|
| x86_64-linux | `x86_64-unknown-linux-musl` | `nix build .#static` |
| aarch64-linux | `aarch64-unknown-linux-musl` | `nix build .#static` |

Static binaries:
- Have no runtime dependencies (no glibc required)
- Can run on any Linux distribution
- Are ideal for distribution and deployment
- Are slightly larger than dynamic binaries

### Phase 0 Tasks

| Task | Description | Output |
|------|-------------|--------|
| T0-001 | Create `flake.nix` with nixos-25.11, naersk, rust-overlay, musl targets | `flake.nix` |
| T0-002 | Create `nix/devshell.nix` with Rust toolchain and dev tools | `nix/devshell.nix` |
| T0-003 | Create `nix/treefmt.nix` for formatting | `nix/treefmt.nix` |
| T0-004 | Initialize Cargo.toml with project metadata | `Cargo.toml` |
| T0-005 | Create minimal `src/main.rs` for build validation | `src/main.rs` |
| T0-006 | Run `nix develop` and verify toolchain | Shell works |
| T0-007 | Run `nix build` and verify dynamic package builds | Package builds |
| T0-008 | Run `nix build .#static` and verify static binary (x86_64-linux) | Static binary, no glibc |
| T0-009 | Verify static binary with `ldd` shows "not a dynamic executable" | No dynamic deps |
| T0-010 | Run `nix fmt` and verify formatting works | Format works |

**Checkpoint**: `nix develop`, `nix build`, `nix build .#static`, and `nix fmt` all succeed. Static binary verified with `ldd`.

---

## Implementation Status

*Updated: 2025-12-26 (late session)*

| Phase | Bead ID | Status | Notes |
|-------|---------|--------|-------|
| Setup | cclv-07v.1 | ✅ Complete | Nix flake, Cargo.toml, dev shell |
| Foundational | cclv-07v.2 | ✅ Complete | Core types, parser, test fixtures |
| US1 - Live Monitoring | cclv-07v.3 | ✅ Complete | File tailing, stdin, split panes, tabs |
| US2 - Session Analysis | cclv-07v.4 | ✅ Complete | Markdown, syntax highlighting, expand/collapse |
| US3 - Usage Statistics | cclv-07v.5 | ✅ Complete | Token counts, cost estimation, filtering |
| US4 - Keyboard Navigation | cclv-07v.6 | ✅ Complete | Key bindings, focus cycling, shortcuts |
| US5 - Search | cclv-07v.7 | ✅ Complete | Search state machine, highlighting, navigation |
| Polish | cclv-07v.8 | 🔄 In Progress | 2 tasks remain: theme selection, snapshot tests |
| **Line Wrapping** | cclv-07v.9 | 🔄 In Progress | Core + per-entry done; **section-level rendering (cclv-07v.9.20) pending** |
| **Logging Pane** | cclv-07v.9.17 | ✅ Complete | Toggleable bottom panel, ring buffer, severity badges |

---

## Phase: Line Wrapping Feature

**Purpose**: Add toggleable line-wrapping behavior with global config and per-item overrides.

**Requirements** (from spec clarifications 2025-12-26):
- FR-039: Toggleable line-wrapping with configurable global default (wrap enabled when unset)
- FR-040: When wrapping disabled, horizontal scrolling with left/right arrow keys
- FR-048: Per-conversation-item wrap toggle overrides global setting
- FR-049: Per-item wrap state is ephemeral (not persisted)
- FR-050: Default keybindings: `w` (per-item), `W` (global)
- FR-051: Global wrap state displayed in status bar
- FR-052: Wrapped lines show continuation indicator (`↩`) at wrap points
- FR-053: Code blocks never wrap (always horizontal scroll)

### Design Decisions

| Aspect | Decision | Rationale |
|--------|----------|-----------|
| Default behavior | Wrap enabled | More readable for prose; power users can disable |
| Code blocks | Never wrap | Code semantics depend on line boundaries |
| Per-item state | Ephemeral `HashSet<EntryUuid>` | No persistence needed; mirrors expand/collapse pattern |
| Visual indicator | `↩` at wrap points | Distinguishes wrap breaks from intentional line breaks |

### Data Model Additions

```rust
// ===== src/state/app_state.rs additions =====

/// Global wrap configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapMode {
    Wrap,
    NoWrap,
}

impl Default for WrapMode {
    fn default() -> Self {
        WrapMode::Wrap  // FR-039: default to wrap enabled
    }
}

// Add to AppState:
pub struct AppState {
    // ... existing fields ...
    pub global_wrap: WrapMode,
}

// Add to ScrollState:
pub struct ScrollState {
    // ... existing fields ...
    /// Messages with wrap override (opposite of global setting)
    pub wrap_overrides: HashSet<EntryUuid>,
}

impl ScrollState {
    /// Toggle wrap for a specific message
    pub fn toggle_wrap(&mut self, uuid: &EntryUuid) {
        if self.wrap_overrides.contains(uuid) {
            self.wrap_overrides.remove(uuid);
        } else {
            self.wrap_overrides.insert(uuid.clone());
        }
    }

    /// Get effective wrap mode for a message
    pub fn effective_wrap(&self, uuid: &EntryUuid, global: WrapMode) -> WrapMode {
        if self.wrap_overrides.contains(uuid) {
            match global {
                WrapMode::Wrap => WrapMode::NoWrap,
                WrapMode::NoWrap => WrapMode::Wrap,
            }
        } else {
            global
        }
    }
}

// ===== src/model/key_action.rs additions =====

pub enum KeyAction {
    // ... existing variants ...

    // Line wrapping (new)
    ToggleWrap,       // Per-item toggle (w)
    ToggleGlobalWrap, // Global toggle (W)
}
```

### Tasks

| Bead | Task | Description | Status |
|------|------|-------------|--------|
| cclv-07v.9.1 | LW-001 | Add `WrapMode` enum to `src/state/app_state.rs` | ✅ Complete |
| cclv-07v.9.2 | LW-002 | Add `wrap_overrides: HashSet<EntryUuid>` to `ScrollState` | ✅ Complete |
| cclv-07v.9.3 | LW-003 | Add `global_wrap` field to `AppState` | ✅ Complete |
| cclv-07v.9.4 | LW-004 | Add `ToggleWrap`, `ToggleGlobalWrap` to `KeyAction` enum | ✅ Complete |
| cclv-07v.9.5 | LW-005 | Add default keybindings: `w` → ToggleWrap, `W` → ToggleGlobalWrap | ✅ Complete |
| cclv-07v.9.6 | LW-006 | Add `line_wrap` config option to `AppConfig` with default `true` | ✅ Complete |
| cclv-07v.9.7 | LW-007 | Implement wrap state handlers in key event processing | ✅ Complete |
| cclv-07v.9.8 | LW-008 | Update message rendering to respect wrap mode | ✅ Complete |
| cclv-07v.9.9 | LW-009 | Add continuation indicator (`↩`) rendering at wrap points | ✅ Complete |
| cclv-07v.9.10 | LW-010 | Exempt code blocks from wrapping (entry-level) | ✅ Complete |
| cclv-07v.9.11 | LW-011 | Display global wrap state in status bar | ✅ Complete |
| cclv-07v.9.12 | LW-012 | Add tests for wrap state transitions | ✅ Complete |
| cclv-07v.9.13 | LW-013 | Add tests for wrap rendering behavior | ✅ Complete |
| cclv-07v.9.14 | LW-014 | Per-entry Paragraph architecture refactor | ✅ Complete |
| cclv-07v.9.17 | LW-015 | Logging pane feature (FR-054–FR-060) | ✅ Complete |
| cclv-07v.9.20 | LW-016 | **Section-level rendering** (see below) | 🔲 Open |

**Checkpoint**: All wrap-related tests pass; visual verification of wrap indicator and code block exemption.

### Known Issues (Blocking)

| Bead | Priority | Description | Status |
|------|----------|-------------|--------|
| cclv-07v.9.15 | P0 | Tests hang waiting for user input | ✅ Fixed |
| cclv-07v.9.16 | P1 | Errors parsing cc-session-log.jsonl (missing sessionId) | ✅ Fixed |

### View Architecture Refactor for Per-Item Wrap (cclv-07v.9.14) ✅ COMPLETE

**Status**: All 9 subtasks complete. Per-entry Paragraph architecture implemented.

**Implemented Architecture**:
```
render_conversation_view()
  ├── render outer Block (title, border)
  ├── for each visible entry:
  │   ├── calculate Y offset from cumulative heights
  │   ├── get effective_wrap(entry.uuid, global_wrap)
  │   ├── build entry's Vec<Line>
  │   ├── create Paragraph with per-entry wrap setting
  │   └── render Paragraph at calculated offset
  └── handle horizontal scroll per-entry when wrap disabled
```

**Key Files**: `src/view/message.rs` - `render_entry_lines()`, `calculate_entry_layouts()`, `EntryLayout` struct

---

### Section-Level Rendering for Code Block Exemption (cclv-07v.9.20)

**Problem**: FR-053 spec clarified (2025-12-26) that code blocks should NOT wrap while prose SHOULD wrap **within the same entry**. Current implementation uses entry-level logic: if any code block exists in entry, the entire entry doesn't wrap.

**Spec Clarification**:
> "At what granularity should code block wrap exemption apply?" → **Section-level**: each prose block and code block rendered as separate Paragraph widget, allowing code to never wrap while prose follows wrap setting within the same entry.

**Current Architecture (entry-level)**:
```
for each entry:
  if has_code_blocks(entry) → entire entry NoWrap
  else → entry follows wrap setting
  render entry as single Paragraph
```

**Target Architecture (section-level)**:
```
for each entry:
  parse markdown into sections: Vec<ContentSection>
  for each section:
    if CodeBlock → render Paragraph with NoWrap + horizontal offset
    if Prose → render Paragraph with effective_wrap() + wrap indicators
```

#### Approach

1. Create `ContentSection` enum:
```rust
enum ContentSection {
    Prose(Vec<Line<'static>>),
    CodeBlock(Vec<Line<'static>>),
}
```

2. Add `parse_entry_sections()` to split entry markdown into content sections

3. Modify render loop to iterate sections, rendering each as separate Paragraph

4. Update height calculation to sum section heights

5. Apply horizontal offset only to code sections

6. Apply wrap indicators only to prose sections

#### Subtasks

| Task | Description | Dependencies |
|------|-------------|--------------|
| LW-016.1 | Create `ContentSection` enum type | None |
| LW-016.2 | Implement `parse_entry_sections()` markdown splitter | LW-016.1 |
| LW-016.3 | Update render loop for per-section Paragraphs | LW-016.2 |
| LW-016.4 | Update height calculation for section sums | LW-016.2 |
| LW-016.5 | Apply horizontal offset to code sections only | LW-016.3 |
| LW-016.6 | Apply wrap indicators to prose sections only | LW-016.3 |
| LW-016.7 | Update search highlighting for section-level rendering | LW-016.3 |
| LW-016.8 | Add tests for mixed prose/code entries | LW-016.3 |

#### Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Markdown parsing edge cases | Reuse existing `has_code_blocks()` logic, extend to extract boundaries |
| Height calculation complexity | Property tests: entry height == sum of section heights |
| Visual continuity between sections | Zero padding between sections within entry |

**Checkpoint**: Entry with both code and prose: code doesn't wrap, prose wraps (if enabled). Wrap toggle affects prose sections only. Horizontal scroll affects code sections only.

---

## Phase: Logging Pane Feature

**Purpose**: Add a toggleable logging pane to display tracing output, preventing errors from breaking the main UI.

**Requirements** (from spec clarifications 2025-12-26):
- FR-054: Toggleable logging pane as a bottom panel
- FR-055: Display tracing output, log level controlled via tracing infrastructure (RUST_LOG / config)
- FR-056: Ring buffer with configurable capacity (default: 1000 entries)
- FR-057: Status bar badge showing unread log count, color-coded by severity
- FR-058: Clear unread count when user opens logging pane
- FR-059: Errors in logging pane MUST NOT interrupt main UI flow
- FR-060: Default keybinding: `L` for ToggleLogPane

### Design Decisions

| Aspect | Decision | Rationale |
|--------|----------|-----------|
| Pane location | Bottom panel | Standard pattern for logs/consoles in dev tools |
| Toggle key | `L` | Mnemonic for "Log", consistent with single-letter shortcuts |
| Log source | Rust tracing | Standard infrastructure; RUST_LOG controls verbosity |
| Buffer type | Ring buffer | Bounded memory; oldest entries dropped when full |
| Capacity default | 1000 entries | Sufficient for diagnosis without unbounded growth |
| Error indication | Status bar badge | Non-intrusive but always visible; color-coded severity |

### Data Model Additions

```rust
// ===== src/state/app_state.rs additions =====

/// Log entry for the logging pane
#[derive(Debug, Clone)]
pub struct LogPaneEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: tracing::Level,
    pub message: String,
}

/// Logging pane state
#[derive(Debug)]
pub struct LogPaneState {
    /// Ring buffer of log entries
    pub entries: VecDeque<LogPaneEntry>,
    /// Maximum entries to retain (configurable)
    pub capacity: usize,
    /// Count of unread entries since pane was last opened
    pub unread_count: usize,
    /// Highest severity among unread entries
    pub unread_max_level: Option<tracing::Level>,
    /// Whether the pane is currently visible
    pub visible: bool,
}

impl LogPaneState {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(capacity),
            capacity,
            unread_count: 0,
            unread_max_level: None,
            visible: false,
        }
    }

    pub fn push(&mut self, entry: LogPaneEntry) {
        if self.entries.len() >= self.capacity {
            self.entries.pop_front();
        }
        if !self.visible {
            self.unread_count += 1;
            self.unread_max_level = Some(
                self.unread_max_level
                    .map_or(entry.level, |l| std::cmp::max(l, entry.level))
            );
        }
        self.entries.push_back(entry);
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible;
        if self.visible {
            self.unread_count = 0;
            self.unread_max_level = None;
        }
    }
}

// Add to AppState:
pub struct AppState {
    // ... existing fields ...
    pub log_pane: LogPaneState,
}

// ===== src/model/key_action.rs additions =====

pub enum KeyAction {
    // ... existing variants ...

    // Logging pane (new)
    ToggleLogPane,    // Toggle log pane visibility (L)
}

// ===== src/config/mod.rs additions =====

pub struct AppConfig {
    // ... existing fields ...
    /// Maximum log entries to retain in logging pane (default: 1000)
    pub log_buffer_capacity: usize,
}
```

### Tasks

| Bead | Description | Dependencies |
|------|-------------|--------------|
| cclv-07v.9.17.1 | Add `LogPaneEntry` and `LogPaneState` types | None |
| cclv-07v.9.17.2 | Add `ToggleLogPane` to `KeyAction` enum | None |
| cclv-07v.9.17.3 | Add default keybinding: `L` → ToggleLogPane | cclv-07v.9.17.2 |
| cclv-07v.9.17.4 | Add `log_buffer_capacity` config option (default: 1000) | None |
| cclv-07v.9.17.5 | Add `log_pane` field to `AppState` | cclv-07v.9.17.1 |
| cclv-07v.9.17.6 | Create custom tracing subscriber that writes to LogPaneState | cclv-07v.9.17.1 |
| cclv-07v.9.17.7 | Implement log pane toggle handler in key event processing | cclv-07v.9.17.5, cclv-07v.9.17.3 |
| cclv-07v.9.17.8 | Create `LogPaneView` widget for rendering log entries | cclv-07v.9.17.1 |
| cclv-07v.9.17.9 | Update layout to include bottom panel when log pane visible | cclv-07v.9.17.8 |
| cclv-07v.9.17.10 | Add unread badge to status bar (count + color by severity) | cclv-07v.9.17.5 |
| cclv-07v.9.17.11 | Add `FocusLogPane` to focus cycling | cclv-07v.9.17.5 |
| cclv-07v.9.17.12 | Add tests for LogPaneState (push, capacity, unread tracking) | cclv-07v.9.17.1 |
| cclv-07v.9.17.13 | Add tests for log pane toggle and focus | cclv-07v.9.17.7 |

**Checkpoint**: Log pane toggles correctly; tracing output appears in pane; status bar shows unread count with correct severity color.

---

## Generated Artifacts

| Artifact | Path | Status |
|----------|------|--------|
| Implementation Plan | specs/001-claude-code-log-viewer/plan.md | ✅ Complete |
| Research Decisions | specs/001-claude-code-log-viewer/research.md | ✅ Complete |
| Data Model | specs/001-claude-code-log-viewer/data-model.md | ✅ Complete |
| CLI Contract | specs/001-claude-code-log-viewer/contracts/cli.md | ✅ Complete |
| Quickstart Guide | specs/001-claude-code-log-viewer/quickstart.md | ✅ Complete |
| Nix Flake | flake.nix | ✅ Implemented |
| Dev Shell | nix/devshell.nix | ✅ Implemented |
| Formatter Config | nix/treefmt.nix | ✅ Implemented |
| Source Code | src/ | ✅ Core complete, polish in progress |

---

## Next Steps

1. **Complete Section-Level Rendering (cclv-07v.9.20)** - NEW priority from spec clarification:
   - LW-016.1: Create `ContentSection` enum type
   - LW-016.2: Implement `parse_entry_sections()` markdown splitter
   - LW-016.3: Update render loop for per-section Paragraphs
   - LW-016.4: Update height calculation for section sums
   - LW-016.5: Apply horizontal offset to code sections only
   - LW-016.6: Apply wrap indicators to prose sections only
   - LW-016.7: Update search highlighting for section-level rendering
   - LW-016.8: Add tests for mixed prose/code entries

2. **Complete Polish phase (cclv-07v.8)** - Remaining tasks:
   - cclv-07v.8.7: Add theme selection support
   - cclv-07v.8.9: Add snapshot tests for key views

### Recently Completed

- ✅ **Line Wrapping core** (cclv-07v.9.1–9.13): WrapMode, per-item toggle, wrap indicators, status bar
- ✅ **Per-entry Paragraph refactor** (cclv-07v.9.14): Architecture supports per-item wrap settings
- ✅ **Logging Pane** (cclv-07v.9.17): Toggleable bottom panel, ring buffer, severity badges
- ✅ **Config file loading** (cclv-07v.8.8): TOML config with precedence chain
- ✅ **Code block exemption** (cclv-07v.9.10): Entry-level (to be refined by cclv-07v.9.20)

