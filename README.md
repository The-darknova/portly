# Portly

> Cross-platform port and process manager with unified CLI and Tauri v2 desktop GUI.

Portly lets developers and system administrators instantly see what is listening on their network ports, identify the exact process and its working directory (CWD), and take immediate action: kill the process in one click, launch the service in a browser, or open a terminal right inside its project folder.

## Key Features

- **Instant Socket & Port Inspection**: Scans TCP and UDP sockets across IPv4 and IPv6 without shell lag.
- **Working Directory (CWD) Resolution**: Reveals where the process is running on disk (e.g. `/home/user/projects/my-web-app`).
- **1-Click Process Termination**: Safe graceful termination (`SIGTERM`) with forced termination (`SIGKILL`) fallback and desktop privilege elevation (`pkexec` on Linux, UAC on Windows).
- **1-Click Quick Actions**: Open `http://localhost:<port>` directly in your browser, or open an external terminal navigated straight to the process CWD.
- **Dual-Mode Binary**:
  - Run `portly` without arguments to launch the lightweight Tauri v2 desktop GUI and system tray.
  - Run `portly list`, `portly find 3000`, `portly kill 3000`, or `portly open 3000` to run headlessly in your terminal with `--json` support.
- **Lightweight & Cross-Platform**: Built with Rust, Tauri v2, and React 19 for Linux, macOS, and Windows.

## Quick Start

### Installation & CLI Usage
```bash
# List all active listening ports
portly list

# Find what is running on port 3000
portly find 3000

# Terminate process holding port 3000
portly kill 3000

# Open port in browser
portly open 3000

# Launch desktop GUI
portly gui
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
