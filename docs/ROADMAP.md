# Portly Roadmap & Development Plan

This document outlines the phased roadmap for **Portly**, tracking completed phases and future milestones.

---

## Completed Phases

### Phase 0: Workspace Scaffolding & AI Governance
- [x] Initialized Git repository and configured Cargo + PNPM workspace.
- [x] Enforced L8 agent discipline and mandatory Conventional Commits ([`AGENTS.md`](../AGENTS.md), [`CLAUDE.md`](../CLAUDE.md)).
- [x] Established architectural documentation and IPC schema in [`docs/`](./).
- [x] Set up granular task specifications in [`tasks/`](../tasks/).

### Phase 1: Core Socket & Process Engine (`crates/portly-core`)
- [x] Implemented live cross-platform TCP/UDP socket enumeration (`netstat2`).
- [x] Built process metadata and CWD discovery (`/proc/[pid]/cwd` on Linux, `sysinfo` on Windows/macOS, `lsof` fallback).
- [x] Implemented system socket inference (53 -> systemd-resolved, 631 -> cupsd, 41641 -> tailscaled).
- [x] Implemented graceful `SIGTERM` and forced `SIGKILL` with desktop elevation (`pkexec` on Linux, UAC on Windows, `osascript` on macOS).
- [x] Built quick action executors (browser, file manager, terminal, code editors).
- [x] Implemented port conflict detection engine for overlapping wildcard and dual-stack bindings.

### Phase 2: Command-Line Interface (`crates/portly-cli`)
- [x] Implemented CLI command suite: `list`, `find`, `kill`, `open`, `cwd`, `conflicts`, `watch`, `config`, `completions`.
- [x] Formatted ASCII output tables via `comfy-table` with colored status indicators.
- [x] Added `--json` streaming output for scripting and pipeline automation.
- [x] Built live interactive terminal monitor (`portly watch --interval 2`).
- [x] Added shell auto-completion generator for Bash, Zsh, Fish, and PowerShell.

### Phase 3: Tauri v2 Desktop Bridge & System Tray (`src-tauri`)
- [x] Configured unified dual-mode binary: launches desktop GUI when run without arguments, runs CLI headlessly with subcommands.
- [x] Bundled frontend assets offline directly from `ui/dist` via `tauri://localhost`.
- [x] Implemented native system tray icon with left-click show/hide toggle and menu (`Show`, `Quit`).
- [x] Built Tauri IPC command handlers for socket scans, conflicts, process termination, actions, and settings.

### Phase 4: React 19 Frontend & Quick Actions (`ui`)
- [x] Scaffolded React 19 + TypeScript + Vite + Tailwind CSS interface.
- [x] Built real-time socket table with instant search and protocol filtering.
- [x] Implemented port conflict alert banner highlighting colliding processes.
- [x] Added 1-click Quick Action buttons per row: browser, terminal, folder, code editor, and graceful/forced termination.

### Phase 5: Packaging & Multi-Platform CI/CD
- [x] Created multi-platform GitHub Actions matrix workflow for Linux, macOS, and Windows.
- [x] Built Arch Linux packaging (`packaging/PKGBUILD`) and desktop launcher entry (`packaging/portly.desktop`).
- [x] Generated multi-resolution PNG32 icon suite (`32x32`, `128x128`, `256x256`, `.ico`, `.icns`).

### Phase 6: Persistent Configuration & Port Pinning
- [x] Built persistent TOML configuration engine (`~/.config/portly/config.toml`).
- [x] Added CLI `portly config` management (editor, terminal, refresh interval, HTTPS, pinned ports).
- [x] Implemented 1-click port pinning (star toggle) with automatic top sorting.
- [x] Built in-app Settings modal for visual preference management.
- [x] Added global keyboard shortcuts (`Ctrl+R` to refresh, `Escape` to close modals/clear search).

---

## Upcoming Phases

### Phase 7: Process Hierarchy & System Tray Enhancements
- [ ] **Tray Quick Menu for Web Ports**: Inject active listening web ports directly into the system tray context menu to launch them in 1 click without opening the window.
- [ ] **Dynamic Tray Tooltip**: Display live active port count in the system tray tooltip (e.g. `Portly: 14 active ports`).
- [ ] **Parent Process Hierarchy (PPID)**: Display parent process tree in an expandable row (e.g. `zsh` -> `npm` -> `vite` -> `esbuild`).
- [ ] **Real-Time Resource Metrics**: Display memory RSS (in MB) and CPU% next to the process name in both CLI and GUI.
- [ ] **Quick Filter Chips**: Add one-click filter pills above the socket table (`Web Only`, `TCP Only`, `UDP Only`, `Conflicts Only`).

### Phase 8: Package Manager Distribution
- [ ] Submit `portly-bin` to Arch User Repository (AUR).
- [ ] Publish Homebrew tap for macOS (`brew install danny/tap/portly`).
- [ ] Publish Windows Winget and Scoop manifests.
- [ ] Publish `portly-cli` and `portly-core` to crates.io (`cargo install portly-cli`).
