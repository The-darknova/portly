# Task 06: Tauri v2 Backend & System Tray

## Objective
Wire `portly-core` into the Tauri v2 desktop application, handle dual-mode binary dispatch, and configure system tray monitoring.

## Acceptance Criteria
- [ ] Check CLI arguments in `src-tauri/src/main.rs`: if arguments are passed, route directly to `portly-cli` without initializing WebView.
- [ ] Implement Tauri IPC commands: `scan_ports`, `kill_process`, `execute_action`.
- [ ] Configure native system tray with:
  - Quick summary of active listening ports count.
  - Quick action menu to bring Portly window to focus or quit.
- [ ] Implement background conflict detection emitting `port-conflict-detected` events.
