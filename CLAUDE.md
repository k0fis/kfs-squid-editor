# CLAUDE.md

## Overview

TUI editor for Squid proxy configuration files. Manages the "access control" layer of squid.conf: ACLs, http_access rules, auth_param basic, and always/never_direct directives.

Written in Rust with ratatui + crossterm. Distributed as static binaries via GitHub Releases.

## Build & Test

```bash
cargo build                    # Dev build
cargo build --release          # Release build
cargo test                     # 14 unit tests (parser, writer, model)
cargo clippy -- -D warnings    # Lint
cargo fmt --check              # Format check
```

## Architecture

```
src/
  main.rs       — CLI (clap), terminal setup, event loop
  model.rs      — Data structures: AclType, Acl, AclRef, AccessRule, DirectRule, AuthParamBasic, SquidConfig
  parser.rs     — Line-by-line squid.conf parser (strips comments, merges same-name ACLs)
  writer.rs     — Serializes SquidConfig back to clean squid.conf text
  help.rs       — Static help strings per ACL type and screen
  app.rs        — App state machine, Screen/Tab enums, all keyboard handling
  ui/
    mod.rs      — Draw dispatch, tab bar, status bar, confirm dialogs
    rules.rs    — Combined ACL (top) + http_access (bottom) split panel
    acl_edit.rs — ACL add/edit form with type help sidebar
    access_edit.rs — Access/Direct rule editor (action toggle + ACL picker)
    auth.rs     — auth_param basic 4-field form
    direct.rs   — always_direct / never_direct split panel
    help_popup.rs — Modal help overlay
```

## Key Design Decisions

- **Three tabs**: Rules (ACLs + http_access in one split panel), Auth, Direct
- **Tab** switches focus between sub-panels within a tab; **Esc** goes to next tab
- **Parser**: skips unknown directives, merges ACLs with same name (OR values)
- **Writer**: clean output, no comments, skips predefined ACLs (all, manager, localhost, etc.)
- **No async**: synchronous crossterm event polling with 100ms timeout
- **Auth**: only `auth_param basic` (program, children, realm, credentialsttl)

## Tabs & Navigation

- **Rules tab**: Top panel = ACL list, Bottom panel = http_access rules. Tab switches focus.
- **Auth tab**: Direct form editing. Tab cycles fields, F2 saves.
- **Direct tab**: Top = always_direct, Bottom = never_direct. Tab switches focus.
- Global: Ctrl+s save, Ctrl+q/q quit, ? help, Esc next tab, Shift+Tab prev tab

## CI/CD

- `.github/workflows/ci.yml` — Push/PR: fmt + clippy + test + build
- `.github/workflows/release.yml` — Tag `v*`: builds 4 targets (linux amd64/arm64, macOS amd64/arm64) + .deb, creates GitHub Release
- `update-kfs-squid-editor.sh` — Curl-based install/update script

## Current State (v0.1.1)

### Done
- Full parser/writer with round-trip tests
- All 23 ACL types supported
- TUI with all screens functional
- ACL add/edit/delete
- http_access add/edit/delete/reorder
- auth_param basic editing
- always_direct / never_direct management
- Context-sensitive help
- Save/load config files
- Dirty-check on quit
- CI + release pipeline

### TODO (future)
- Input cursor position (currently appends only, no arrow key movement in text fields)
- Multiline text input with scrolling (for ACL values)
- Input validation (IP format, port ranges, time format)
- Search/filter in ACL list
- Undo/redo
- Config file backup before overwrite
- Real-world testing with production squid.conf files
