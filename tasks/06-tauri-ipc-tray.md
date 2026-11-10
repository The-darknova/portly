# Task 06: Tauri v2 Backend & System Tray

## Objective
Wire `portly-core` into the Tauri v2 desktop application, handle dual-mode binary dispatch, and configure system tray monitoring.

## Acceptance Criteria
- [x] Check CLI arguments in `src-tauri/src/main.rs`: if arguments are passed, route directly to `portly-cli` without initializing WebView.
- [x] Implement Tauri IPC commands: `scan_ports`, `detect_conflicts`, `kill_process`, `execute_action`.
- [x] Configure native system tray with:
  - Menu to Show Portly window or Quit.
  - Left-click tray icon to toggle focus.
- [x] Implement background conflict detection and commands.
