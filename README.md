# Portly

> Cross-platform port and process manager with unified CLI and Tauri v2 desktop GUI.

Portly lets developers and system administrators instantly see what is listening on their network ports, identify the exact process and its working directory (CWD), and take immediate action: kill the process in one click, launch the service in a browser, or open a terminal right inside its project folder.

## Key Features

- **Instant Socket & Port Inspection**: Scans TCP and UDP sockets across IPv4 and IPv6 without shell lag.
- **Working Directory (CWD) Resolution**: Reveals where the process is running on disk (e.g. `/home/user/projects/my-web-app`).
- **1-Click Process Termination**: Safe graceful termination (`SIGTERM`) with forced termination (`SIGKILL`) fallback and desktop privilege elevation (`pkexec` on Linux, UAC on Windows).
- **Port Conflict Detection**: Detects port collisions (e.g., both `0.0.0.0:8080` and `127.0.0.1:8080` bound by conflicting processes).
- **1-Click Quick Actions**: Open `http://localhost:<port>` directly in your browser, open your favorite code editor (VS Code, Cursor, Zed) at the project root, or launch a terminal navigated straight to the process CWD.
- **Dual-Mode Binary**:
  - Run `portly` without arguments to launch the lightweight Tauri v2 desktop GUI and system tray.
  - Run `portly list`, `portly find 3000`, `portly kill 3000`, `portly open 3000`, `portly cwd 3000`, `portly watch`, `portly conflicts`, or `portly completions` to run headlessly in your terminal with `--json` support.
- **Lightweight & Cross-Platform**: Built with Rust, Tauri v2, and React 19 for Linux, macOS, and Windows.

## Quick Start

### Installation & CLI Usage
```bash
# List all active listening ports in a formatted table
portly list

# List in JSON format for scripts or automation pipelines
portly list --json

# Find what is running on port 3000
portly find 3000

# Print the working directory of the process holding port 3000
portly cwd 3000

# Terminate process holding port 3000 gracefully (or with --force, --elevated)
portly kill 3000
portly kill 3000 --force --elevated

# Open port in your default browser
portly open 3000

# Check for dual-stack or wildcard port collisions
portly conflicts

# Live terminal monitor mode (auto-refreshes every 2 seconds)
portly watch --interval 2

# Generate shell completions for your shell
portly completions zsh > ~/.zsh/completion/_portly
portly completions fish > ~/.config/fish/completions/portly.fish
portly completions bash > /etc/bash_completion.d/portly

# Launch desktop GUI
portly
```

### Development Setup
```bash
# Install frontend dependencies
pnpm install

# Run CLI locally
cargo run -p portly-cli -- list

# Run desktop GUI in dev mode
pnpm tauri dev
```

## Documentation
- [Architecture & Internals](docs/ARCHITECTURE.md)
- [Privilege Escalation Specification](docs/PRIVILEGE-ESCALATION.md)
- [IPC Contract](docs/IPC-CONTRACT.md)
- [OS Support Matrix](docs/OS-SUPPORT.md)
- [AI Contributor Guidelines](AGENTS.md)
