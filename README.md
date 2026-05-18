# kfs-squid-editor

Terminal UI editor for Squid proxy configuration files.

## Features

- **ACL Management** — Create, edit, and delete Access Control Lists with support for all 23 Squid ACL types (src, dst, dstdomain, url_regex, port, proto, method, time, arp, proxy_auth, and more)
- **http_access Rules** — Build ordered access rules by combining ACLs with allow/deny actions. Reorder rules with ease (order matters in Squid!)
- **Authentication** — Configure `auth_param basic` settings (program, children, realm, credentials TTL)
- **Direct Routing** — Manage `always_direct` and `never_direct` rules for cache peer routing
- **Built-in Help** — Context-sensitive help explaining each ACL type and directive
- **Parse & Write** — Load existing `squid.conf` files and write clean output
- **Input Validation** — Validates ACL values by type (IP/CIDR format, port ranges, time format, MAC addresses, protocols, HTTP methods)
- **Undo/Redo** — Full undo/redo with 50-level history (Ctrl+Z / Ctrl+Y)
- **Search/Filter** — Filter ACL list by name or type (`/` key)
- **Backup** — Automatically creates `.conf.bak` backup before overwriting

## Installation

### Auto-update script

```bash
curl -sL https://raw.githubusercontent.com/k0fis/kfs-squid-editor/main/update-kfs-squid-editor.sh | bash
```

Detects your OS and architecture, downloads the correct binary from the latest release.

### From GitHub Releases

Download the latest binary for your platform from [Releases](https://github.com/k0fis/kfs-squid-editor/releases).

**Linux (amd64):**
```bash
curl -sL https://github.com/k0fis/kfs-squid-editor/releases/latest/download/kfs-squid-editor-linux-amd64 -o /usr/local/bin/kfs-squid-editor
chmod +x /usr/local/bin/kfs-squid-editor
```

**Debian/Ubuntu (.deb):**
```bash
# Check releases page for the latest .deb filename
curl -sLO https://github.com/k0fis/kfs-squid-editor/releases/latest/download/kfs-squid-editor_0.2.0-1_amd64.deb
sudo dpkg -i kfs-squid-editor_0.2.0-1_amd64.deb
```

**macOS (Homebrew):**
```bash
brew install k0fis/tap/kfs-squid-editor
```

### From source
```bash
cargo install --path .
```

## Usage

```bash
# Edit default /etc/squid/squid.conf
sudo kfs-squid-editor

# Edit a specific file
kfs-squid-editor /path/to/squid.conf

# Create a new configuration from scratch
kfs-squid-editor ./new-squid.conf
```

## Key Bindings

### Global
| Key | Action |
|-----|--------|
| `Ctrl+s` | Save configuration to file |
| `Ctrl+q` or `q` | Quit (confirms if unsaved changes) |
| `Ctrl+z` | Undo |
| `Ctrl+y` | Redo |
| `?` or `F1` | Toggle help |
| `Esc` | Next tab |
| `Shift+Tab` | Previous tab |

### Navigation (Rules tab)
| Key | Action |
|-----|--------|
| `Tab` | Switch focus between ACL panel (top) and Access panel (bottom) |
| `j` / `k` or `↑` / `↓` | Move selection |
| `/` | Search/filter ACLs (by name or type) |

### List Operations
| Key | Action |
|-----|--------|
| `a` | Add new item |
| `e` or `Enter` | Edit selected item |
| `d` | Delete selected (with confirmation) |
| `u` / `J` | Move rule up/down (Access and Direct panels) |

### Edit Mode
| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Cycle through fields |
| `F2` | Save changes |
| `Esc` | Cancel |
| `←` / `→` | Navigate within text / cycle ACL type / toggle action |
| `Home` / `End` | Jump to start/end of text field |
| `Ctrl+a` / `Ctrl+e` | Jump to start/end of text field |
| `Ctrl+u` / `Ctrl+k` | Kill text to start/end |
| `Space` | Toggle checkbox / select ACL |
| `!` | Negate selected ACL reference |

### Auth Tab
| Key | Action |
|-----|--------|
| `Tab` | Next field |
| `F2` | Save auth configuration |
| `Esc` | Switch to next tab |

## Layout

```
┌─────────────────────────────────────────┐
│ [ACLs & Rules]  [Auth]  [Direct]   [?]  │  ← Tab bar
├─────────────────────────────────────────┤
│ Name         │ Type       │ Fl │ Values │  ← ACL panel
│ localnet     │ src        │    │ 10.0.… │
│ blocked      │ dstdomain  │    │ .ad.c… │
├─────────────────────────────────────────┤
│ # │ Action │ ACLs (AND logic)           │  ← Access panel
│ 1 │ allow  │ localnet                   │
│ 2 │ deny   │ all                        │
├─────────────────────────────────────────┤
│ a:Add  e:Edit  d:Del  /:Search  ?:Help  │  ← Status bar
└─────────────────────────────────────────┘
```

## Supported ACL Types

src, dst, srcdomain, dstdomain, srcdom_regex, dstdom_regex, url_regex, urlpath_regex, port, proto, method, time, arp, myip, myport, proxy_auth, browser, referer_regex, req_mime_type, rep_mime_type, maxconn, external, snmp_community

## Building

```bash
cargo build --release
cargo test                     # 26 tests
cargo clippy -- -D warnings    # lint
```

## License

MIT
