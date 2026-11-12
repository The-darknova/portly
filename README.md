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

### Installation

Choose the installation method for your platform:

- **Arch Linux / Omarchy**:
  ```bash
  cd packaging && makepkg -si
  ```
- **From Source (Cargo & Rust)**:
  ```bash
  cargo install --path crates/portly-cli
  ```
- **Pre-built Packages**: Download `.deb`, `.AppImage`, `.dmg`, or `.msi` installers from [GitHub Releases](https://github.com/danny/portly/releases).

---

### Command-Line Interface (CLI)

#### 1. Inspecting Listening Ports
```bash
# List all active listening ports in a formatted table
portly list

# Filter by protocol or emit machine-readable JSON for scripting
portly list --proto tcp
portly list --json

# Find a specific port or process by name or number
portly find 3000
portly find node

# Check for port collisions (e.g. 0.0.0.0 vs 127.0.0.1 dual-binds)
portly conflicts

# Live terminal monitor (refreshes every 2 seconds)
portly watch --interval 2
```

#### 2. Quick Actions: Browser, Editor, and Terminal
```bash
# Open listening service in default web browser
portly open 3000

# Print the process working directory (CWD)
portly cwd 3000

# Open process working directory directly in your preferred editor (VS Code, Cursor, Zed)
portly cwd 3000 --editor

# Open a new terminal session at the process working directory
portly cwd 3000 --terminal

# Reveal the folder in your system file manager
portly cwd 3000 --reveal
```

#### 3. Terminating Processes
```bash
# Gracefully terminate the process on port 3000 (SIGTERM)
portly kill 3000

# Forcefully terminate with privilege escalation (pkexec on Linux, UAC on Windows)
portly kill 3000 --force --escalate
```

#### 4. Persistent Configuration
```bash
# View current settings and config file location
portly config show
portly config path

# Configure default editor and terminal
portly config set-editor cursor
portly config set-terminal ghostty

# Pin critical ports to the top of the list
portly config pin 5432
```

#### 5. Shell Auto-Completions
```bash
# Generate shell completions
portly completions zsh > ~/.zsh/completion/_portly
portly completions fish > ~/.config/fish/completions/portly.fish
portly completions bash > /etc/bash_completion.d/portly
```

---

### Desktop GUI & System Tray

Launch the desktop window and background tray monitor:

```bash
# Launch GUI directly from terminal
portly
# or explicitly
portly gui
```

Or launch **Portly** directly from your desktop application launcher (e.g. Rofi, Omarchy Menu, Spotlight, Start Menu).

---

### Development Setup

To build and run Portly from source in a local development environment:

```bash
# 1. Clone the repository and install dependencies
git clone https://github.com/danny/portly.git
cd portly
pnpm install

# 2. Run the CLI directly via Cargo
cargo run -p portly-cli -- list

# 3. Launch the desktop GUI in hot-reloading development mode
pnpm tauri dev

# 4. Run the full test suite and linters
cargo test --all
cargo clippy --all-targets --all-features -- -D warnings
```

## Documentation
- [Contributor Guidelines](CONTRIBUTING.md)
- [Roadmap & Development Plan](docs/ROADMAP.md)
- [Architecture & Internals](docs/ARCHITECTURE.md)
- [Privilege Escalation Specification](docs/PRIVILEGE-ESCALATION.md)
- [IPC Contract](docs/IPC-CONTRACT.md)
- [OS Support Matrix](docs/OS-SUPPORT.md)
- [AI Contributor Guidelines](AGENTS.md)
