# Portly: AI Agent Guidelines & Engineering Manual

Welcome to **Portly**, a high-performance cross-platform port and process manager for Linux, macOS, and Windows. This repository is maintained with pair-programming AI agents and human engineers. All agents operating in this repository MUST follow the rules and engineering discipline outlined below.

---

## 1. Core Operating Principles

1. **Production-Ready by Default (Zero-Mock Rule)**:
   - Never write mock data or toy implementations.
   - Every socket scan must query real operating system networking tables (`netstat2`, `/proc`, `libproc`, Win32 IP Helper).
   - Every process resolution must query live OS process tables and real file systems.
2. **Elevated Privilege Safety**:
   - On Linux, **ALWAYS use `pkexec` instead of `sudo`** when root privileges are required. Never invoke `sudo` in non-interactive agent shells as it hangs on terminal password prompts.
   - Never run destructive actions silently. Terminating a process must default to graceful termination (`SIGTERM`), with fallback or user consent for forced termination (`SIGKILL`).
3. **No Em-Dashes**:
   - Never use em-dashes (`—`) in prose, comments, commit messages, or documentation. Use standard hyphens (`-`) or colons (`:`).
4. **Mandatory Conventional Commits**:
   - All Git commits must strictly adhere to the Conventional Commits specification:
     - `feat(<scope>): <short description>`
     - `fix(<scope>): <short description>`
     - `refactor(<scope>): <short description>`
     - `docs(<scope>): <short description>`
     - `test(<scope>): <short description>`
     - `chore(<scope>): <short description>`
   - Scope should indicate the crate or module: `(core)`, `(cli)`, `(tauri)`, `(ui)`, `(docs)`.
   - Never include em-dashes in commit messages.
5. **Always Test, Provide Evidence & Validate Before Completion**:
   - No task is complete without verification.
   - Every change must provide verifiable output:
     - Rust code: `cargo test --all` and live CLI run demonstration.
     - Frontend code: `pnpm tsc --noEmit` and `pnpm build` output.
     - Provide tangible execution logs as evidence in every progress update.
6. **Architecture Over Shortcut Bias**:
   - Maintain strict separation of concerns:
     - `crates/portly-core`: Standalone, platform-agnostic business logic, socket scanner, process metadata resolver, and quick action executors.
     - `crates/portly-cli`: High-speed command-line interface with formatted tables and JSON streaming.
     - `src-tauri`: Desktop window, system tray daemon, and IPC bridge.
     - `ui`: Modern React 19 + TypeScript + Tailwind CSS interface.

---

## 2. Repository Layout

```text
portly/
├── AGENTS.md               # This operating guide
├── CLAUDE.md               # Fast CLI cheatsheet for terminal agents
├── docs/                   # Architectural RFCs and specifications
├── tasks/                  # Task specifications and acceptance criteria
├── crates/
│   ├── portly-core/        # Cross-platform socket and process engine
│   └── portly-cli/         # CLI binary
├── src-tauri/              # Tauri v2 desktop application and tray
└── ui/                     # React 19 + TypeScript frontend
```

---

## 3. Technology & Tooling Guidelines

- **Rust Version**: 1.80+ (2021 edition).
- **Node & Package Manager**: Node 22+ with `pnpm`.
- **Formatting & Linting**:
  - Rust: `cargo fmt --all && cargo clippy --all-targets --all-features -- -D warnings`
  - TypeScript: `pnpm lint` and `pnpm tsc --noEmit`
- **Tauri Version**: Tauri v2 (`@tauri-apps/api@^2.0`, `@tauri-apps/cli@^2.0`).

---

## 4. Cross-Platform Handling

| Target OS | Socket Inspection | Process & CWD Discovery | Privilege Escalation |
| :--- | :--- | :--- | :--- |
| **Linux** | `netstat2` / `/proc/net/{tcp,udp}` | `/proc/[pid]/cwd`, `/proc/[pid]/exe` | `pkexec` |
| **macOS** | `netstat2` / `sysctl` | `libproc` (`proc_pidpath`, `proc_pidinfo`) | `osascript` / `authopen` |
| **Windows** | Win32 `GetExtendedTcpTable` | `OpenProcess`, `QueryFullProcessImageNameW` | UAC `runas` |
