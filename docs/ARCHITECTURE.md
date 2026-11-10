# Portly Architecture Specification

## Overview

Portly is designed to solve a ubiquitous developer and system administrator problem: quickly identifying what process is holding a port, understanding its context (especially its working directory), and taking immediate action (opening in browser, navigating in terminal, or cleanly terminating it).

## Module Architecture

```text
               +----------------------------------+
               |     Unified Entrypoint (portly)   |
               +-----------------+----------------+
                                 |
           +---------------------+---------------------+
           |                                           |
+----------v-----------+                    +----------v-----------+
|     portly-cli       |                    |       src-tauri      |
| (clap v4 headless)   |                    | (Tauri v2 Desktop)   |
+----------+-----------+                    +----------+-----------+
           |                                           |
           +---------------------+---------------------+
                                 |
                    +------------v------------+
                    |       portly-core       |
                    | (Platform Agnostic)     |
                    +------------+------------+
                                 |
        +------------------------+------------------------+
        |                        |                        |
+-------v-------+        +-------v-------+        +-------v-------+
|  Linux Engine |        | macOS Engine  |        | Windows Engine|
| (/proc, pkexec|        | (libproc, auth|        | (IP Helper,   |
|   netstat2)   |        |   netstat2)   |        |   Win32 UAC)  |
+---------------+        +---------------+        +---------------+
```

### 1. `crates/portly-core`
The core crate contains no UI or CLI logic. It exposes clean, thread-safe asynchronous and synchronous functions:
- `scan_listeners() -> Result<Vec<ListeningSocket>, CoreError>`: Discovers all TCP and UDP listening ports across IPv4 and IPv6.
- `resolve_process_info(pid: u32) -> Result<ProcessInfo, CoreError>`: Resolves executable path, command-line arguments, environment variables, user/owner, and working directory (`cwd`).
- `terminate_process(pid: u32, force: bool) -> Result<(), CoreError>`: Terminates the target process gracefully (`SIGTERM`), with fallback or option for immediate termination (`SIGKILL`).
- `terminate_with_elevation(pid: u32, force: bool) -> Result<(), CoreError>`: Executes elevation mechanism (`pkexec` on Linux, `osascript` on macOS, `runas` on Windows).
- `open_url(url: &str) -> Result<(), CoreError>`: Opens URL in default browser.
- `open_path_in_terminal(path: &Path) -> Result<(), CoreError>`: Spawns the default or installed terminal in the specified directory.
- `open_path_in_file_manager(path: &Path) -> Result<(), CoreError>`: Reveals the target directory in the OS file explorer.

### 2. `crates/portly-cli`
The CLI application wraps `portly-core` and uses `clap` v4 for subcommands:
- `portly list`: Prints pretty formatted table of active ports. Supports `--json`.
- `portly find <target>`: Searches ports and processes by port number or binary name.
- `portly kill <port>`: Kills the process listening on `<port>`. Supports `--force`.
- `portly open <port>`: Opens `http://localhost:<port>` in default browser.
- `portly cwd <port>`: Prints or navigates to the working directory.

### 3. `src-tauri` & `ui`
The desktop application embeds `portly-core` behind Tauri IPC commands. It runs a lightweight background polling loop, manages a system tray icon, and renders a virtualized socket table in React 19.
