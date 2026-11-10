# Privilege Escalation Specification

## Rationale

When a process is launched by `root` (Linux/macOS) or `SYSTEM` / an Administrator (Windows), an unprivileged user process cannot read its working directory (`/proc/[pid]/cwd` has permissions `0700` owned by root) or send termination signals (`EPERM`).

Portly avoids running the entire desktop GUI or CLI as root. Instead, it operates unprivileged by default and escalates privileges only for specific targets when required.

## Platform Implementations

### Linux
1. **Interactive Prompt**:
   - Portly utilizes `pkexec` (PolicyKit) to run atomic helper commands.
   - For reading restricted process CWD:
     `pkexec readlink -f /proc/<pid>/cwd`
   - For terminating restricted process:
     `pkexec kill -15 <pid>` or `pkexec kill -9 <pid>`
   - PolicyKit presents a native desktop authentication dialog (Polkit-gnome, lxqt-policykit, etc.) with biometric or password prompt.
2. **Never Use `sudo`**:
   - `sudo` is banned for non-interactive desktop environments as it attempts to read from a TTY and hangs the application.

### macOS
1. **Authorization Framework**:
   - Uses `osascript` to trigger an administrator authorization dialog:
     `osascript -e 'do shell script "kill -15 <pid>" with administrator privileges'`
   - Or uses macOS Security framework `AuthorizationExecuteWithPrivileges`.

### Windows
1. **User Account Control (UAC)**:
   - Uses `ShellExecuteExW` with the `runas` verb to spawn `taskkill /PID <pid> /F` with an elevated UAC consent prompt.

## Security Safeguards
- Commands executed under elevation are strictly parameterized and validated: only positive integer PIDs are accepted, preventing arbitrary shell command injection.
- Portly never executes raw strings supplied by the user directly in an elevated shell.
