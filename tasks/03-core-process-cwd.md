# Task 03: Process Metadata & Working Directory Resolver

## Objective
For each discovered PID, reliably resolve the process name, full executable path, command-line arguments, process owner/user, and working directory (CWD).

## Acceptance Criteria
- [ ] Implement `process.rs` using `sysinfo` and OS-specific modules.
- [ ] On Linux: resolve CWD via `/proc/[pid]/cwd` symlink.
- [ ] On macOS: resolve CWD via `libproc` (`PROC_PIDVNODEPATHINFO`).
- [ ] On Windows: resolve CWD via process PEB or toolhelp APIs.
- [ ] Fallback gracefully when a process is owned by another user (mark as elevated/restricted without crashing).
- [ ] Unit tests verifying resolution for the current test process and child processes.
