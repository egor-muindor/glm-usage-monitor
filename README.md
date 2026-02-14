# GLM Usage Monitor

Realtime GLM Coding Plan usage monitor with a beautiful Terminal UI built with Rust and ratatui.

## Features

- Realtime monitoring of GLM API quota limits
- Beautiful TUI with colored progress bars
- Support for multiple platforms (Z.ai / Zhipu)
- Configuration via environment variables or config file
- Auto-refresh with configurable interval
- Keyboard shortcuts for manual refresh and quit
- **Status bar mode** (`--status`) for integration with tmux, i3status, Claude Code, etc.
- Smart caching (5-min TTL) to avoid excessive API calls in status mode

## Screenshots

```
┌─ Header ───────────────────────────────────────────┐
│ GLM Usage Monitor | ZAI | https://api.z.ai         │
│ Refresh: 300s | Timeout: 20s                        │
│ Last update: 2025-12-25 12:34:56 | Next in: 145s  │
└────────────────────────────────────────────────────┘
┌─ Quota Limits ─────────────────────────────────────┐
│ TOKEN: 1 234 567/10 000 000 [██████░░░░░░░░░░░░] 12%│
│    Remaining: 8 765 433                            │
│                                                     │
│ REQUEST: 50 000/100 000 [████████████░░░░░░░] 50%  │
│    Remaining: 50 000                               │
└────────────────────────────────────────────────────┘
┌─ Footer ───────────────────────────────────────────┐
│ Keys: r=refresh now q=quit                          │
│ Status: Connected                                   │
└────────────────────────────────────────────────────┘
```

## Installation

### Prerequisites

You need Rust installed. Install it from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Method 1: Install from git

```bash
cargo install --git https://github.com/your-username/glm-usage-monitor.git
```

### Method 2: Install from local directory

```bash
cd glm-usage-monitor
cargo install --path .
```

The binary will be installed to `~/.cargo/bin/glm-usage-monitor` and automatically available in your PATH.

## Quick Start

```bash
# Install (choose one method)
cargo install --git https://github.com/your-username/glm-usage-monitor.git

# Run - only token is required
export ANTHROPIC_AUTH_TOKEN="your-token"
glm-usage-monitor
```

That's it! The monitor will start with default settings:
- Base URL: `https://api.z.ai/api/anthropic`
- Refresh: every 1 minute
- Timeout: 20 seconds

## Configuration

### Environment Variables

```bash
# Optional: default is "https://api.z.ai/api/anthropic"
export ANTHROPIC_BASE_URL="https://api.z.ai/api/anthropic"
# or: export ANTHROPIC_BASE_URL="https://open.bigmodel.cn/api/anthropic"

# Required: your authentication token
export ANTHROPIC_AUTH_TOKEN="your-token-here"

# Optional:
export REFRESH_SEC="60"  # default: 60 (1 minute)
export HTTP_TIMEOUT_SEC="20"  # default: 20 seconds
```

### Config File

Create a config file at `~/.config/glm-usage-monitor/config.toml`:

```toml
[api]
# base_url is optional, defaults to "https://api.z.ai/api/anthropic"
auth_token = "your-token-here"

[status]
# Show TIME_LIMIT when usage >= threshold% (default: 50)
# 0 = always show, 100 = never show
time_threshold = 50
```

**Note:** Environment variables take precedence over config file values.

## Usage

Run the monitor:

```bash
# Minimal - only token is required
ANTHROPIC_AUTH_TOKEN="your-token" glm-usage-monitor

# With custom refresh interval
glm-usage-monitor -r 60  # refresh every 60 seconds

# With custom timeout
glm-usage-monitor -t 30  # 30 second timeout

# Combined
glm-usage-monitor -r 60 -t 30

# Status bar mode (one-line output for scripts/status bars)
glm-usage-monitor --status
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `r` | Refresh data now |
| `q` | Quit |

## Status Bar Mode

Use `--status` for compact one-line output, perfect for integration with status bars:

```bash
glm-usage-monitor --status
# Output: 📊 50%
# Or when time limit exceeds threshold: ⏱ 80% | 📊 50%
```

The status mode uses a 5-minute cache to avoid excessive API calls.

### Claude Code Integration

Add to your Claude Code settings (`~/.claude/settings.json`):

```json
{
  "statusLine": {
    "type": "command",
    "command": "glm-usage-monitor --status"
  }
}
```

Now you'll see your GLM quota usage directly in Claude Code's status line!

### tmux Integration

Add to your `~/.tmux.conf`:

```bash
set -g status-right "#(glm-usage-monitor --status) | %H:%M"
```

## Development

```bash
# Build
cargo build

# Run
cargo run

# Run with environment variables
ANTHROPIC_BASE_URL="https://api.z.ai/api/anthropic" \
ANTHROPIC_AUTH_TOKEN="your-token" \
cargo run

# Run tests
cargo test

# Build release binary
cargo build --release
```

## Project Structure

```
src/
├── main.rs      # Entry point, CLI parsing
├── config.rs    # Configuration loading (ENV + file)
├── models.rs    # Data models and formatting
├── api.rs       # HTTP client for GLM API
├── app.rs       # Application state and logic
├── cache.rs     # Caching layer for status mode
├── ui.rs        # TUI rendering with ratatui
└── terminal.rs  # Terminal management and event loop
```

## License

MIT
