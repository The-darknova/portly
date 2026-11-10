# Task 03: Process Metadata & Working Directory Resolver

## Objective
For each discovered PID, reliably resolve the process name, full executable path, command-line arguments, process owner/user, and working directory (CWD).

## Acceptance Criteria
- [x] Implement `process.rs` using `sysinfo` and OS-specific modules.
- [x] On Linux: resolve CWD via `/proc/[pid]/cwd` symlink.
- [x] On macOS: resolve CWD via `libproc` (`PROC_PIDVNODEPATHINFO`).
- [x] On Windows: resolve CWD via process PEB or toolhelp APIs.
- [x] Fallback gracefully when a process is owned by another user (mark as elevated/restricted without crashing).
- [x] Unit tests verifying resolution for the current test process and child processes.
