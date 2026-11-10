# Task 04: Process Killer & Privilege Escalation

## Objective
Implement safe process termination in `crates/portly-core` with graceful termination (`SIGTERM`), forced termination (`SIGKILL`), and privilege escalation fallback (`pkexec`).

## Acceptance Criteria
- [x] Implement `killer.rs` exposing `terminate_process(pid: u32, force: bool) -> Result<(), CoreError>`.
- [x] Attempt unprivileged graceful termination first.
- [x] If permission denied or if caller specifies `escalate: true`, invoke `pkexec` (Linux), `osascript` (macOS), or UAC `runas` (Windows).
- [x] Parameterize and sanitize PIDs to prevent any possibility of command injection.
- [x] Implement `actions.rs` for launching browser URLs, file managers, editors, and terminals in the process CWD.
