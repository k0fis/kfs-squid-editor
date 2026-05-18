# kfs-squid-editor

Terminal UI editor for Squid proxy configuration files.

## Features

- **ACL Management** — Create, edit, and delete Access Control Lists with support for all Squid ACL types (src, dst, dstdomain, url_regex, port, proto, method, time, arp, proxy_auth, and more)
- **http_access Rules** — Build ordered access rules by combining ACLs with allow/deny actions. Reorder rules with ease (order matters in Squid!)
- **Authentication** — Configure `auth_param basic` settings (program, children, realm, credentials TTL)
- **Direct Routing** — Manage `always_direct` and `never_direct` rules for cache peer routing
- **Built-in Help** — Context-sensitive help explaining each ACL type and directive
- **Parse & Write** — Load existing `squid.conf` files and write clean output

## Installation

### From GitHub Releases

Download the latest binary for your platform from [Releases](https://github.com/k0fis/kfs-squid-editor/releases).

**Linux (amd64):**
```bash
curl -sL https://github.com/k0fis/kfs-squid-editor/releases/latest/download/kfs-squid-editor-linux-amd64 -o /usr/local/bin/kfs-squid-editor
chmod +x /usr/local/bin/kfs-squid-editor
```

**Debian/Ubuntu (.deb):**
```bash
curl -sLO https://github.com/k0fis/kfs-squid-editor/releases/latest/download/kfs-squid-editor_0.1.0-1_amd64.deb
sudo dpkg -i kfs-squid-editor_0.1.0-1_amd64.deb
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

### Navigation
| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Switch between sections (ACLs, Access, Auth, Direct) |
| `j` / `k` or `↑` / `↓` | Move selection |
| `?` or `F1` | Toggle help |

### List Operations
| Key | Action |
|-----|--------|
| `a` | Add new item |
| `e` or `Enter` | Edit selected item |
| `d` | Delete selected (with confirmation) |
| `u` / `J` | Move rule up/down (Access and Direct tabs) |

### Edit Mode
| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Cycle through fields |
| `F2` | Save changes |
| `Esc` | Cancel |
| `←` / `→` | Cycle ACL type or toggle action |
| `Space` | Toggle checkbox / select ACL |
| `!` | Negate selected ACL reference |

### Global
| Key | Action |
|-----|--------|
| `Ctrl+s` | Save configuration to file |
| `Ctrl+q` or `q` | Quit (confirms if unsaved changes) |

## Supported ACL Types

src, dst, srcdomain, dstdomain, srcdom_regex, dstdom_regex, url_regex, urlpath_regex, port, proto, method, time, arp, myip, myport, proxy_auth, browser, referer_regex, req_mime_type, rep_mime_type, maxconn, external, snmp_community

## Building

```bash
cargo build --release
```

## License

MIT
