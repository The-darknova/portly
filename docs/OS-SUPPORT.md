# Operating System Support Matrix

This document outlines platform capabilities, APIs, and limitations across Linux, macOS, and Windows.

| Feature | Linux | macOS | Windows |
| :--- | :--- | :--- | :--- |
| **Port Discovery** | `/proc/net/tcp{,6}` & `netstat2` | `netstat2` & `sysctl` | Win32 IP Helper (`GetExtendedTcpTable`) |
| **PID Association** | Inode resolution via `/proc/[pid]/fd` | `libproc` socket info | Directly provided by IP Helper table |
| **Process Name** | `/proc/[pid]/comm` or `cmdline` | `proc_pidpath` | `QueryFullProcessImageNameW` |
| **Working Directory (CWD)** | `readlink /proc/[pid]/cwd` | `proc_pidinfo` (`PROC_PIDVNODEPATHINFO`) | PEB via `NtQueryInformationProcess` |
| **Kill (Graceful)** | `kill(pid, SIGTERM)` | `kill(pid, SIGTERM)` | `PostMessage(hwnd, WM_CLOSE)` |
| **Kill (Forced)** | `kill(pid, SIGKILL)` | `kill(pid, SIGKILL)` | `TerminateProcess(hProcess, 1)` |
| **Elevation** | `pkexec` | `osascript` / Security Framework | UAC `runas` |
| **Open in Terminal** | Auto-detects: `ghostty`, `alacritty`, `kitty`, `wezterm`, `gnome-terminal`, `xterm` | `open -a Terminal <dir>` or `open -a iTerm <dir>` | `wt.exe -d <dir>` or `cmd.exe /k cd /d <dir>` |
| **Open in Explorer** | `xdg-open <dir>` | `open <dir>` | `explorer.exe <dir>` |

## Known Caveats
1. **Linux Containers & Namespaces**: Sockets inside Docker containers or isolated network namespaces are visible only if scanning from the host with sufficient capabilities (`CAP_NET_ADMIN`).
2. **macOS App Sandbox**: To inspect arbitrary system sockets and processes on macOS, the application cannot run inside the restrictive App Store sandbox and must be distributed outside the Mac App Store with Developer ID notarization.
3. **Windows Antivirus**: Heuristic scanners may flag reading process PEBs. Windows APIs are invoked with minimal required access rights (`PROCESS_QUERY_LIMITED_INFORMATION`).
