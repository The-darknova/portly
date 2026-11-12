# Contributing to Portly

Thank you for your interest in contributing to **Portly**! Portly is an open-source, high-performance cross-platform port and process manager built with Rust, Tauri v2, and React 19.

This guide outlines our development workflow, engineering standards, and pull request procedures for both human engineers and AI contributor agents.

---

## 1. Code of Conduct & Community Ethos

- **Be Respectful**: Treat everyone with courtesy and professional respect.
- **Production-Grade Quality**: Avoid temporary hacks, toy implementations, or mock data. We build maintainable software from day one.
- **Transparent Communication**: Clearly explain the rationale, trade-offs, and verification steps in issues and pull requests.

---

## 2. Repository Layout & Architecture

Portly enforces a strict separation of concerns across a Cargo workspace and a frontend package:

```text
portly/
├── crates/
│   ├── portly-core/        # Platform-agnostic engine (socket scanning, process inspection, actions)
│   └── portly-cli/         # High-performance command-line tool with table and JSON streaming
├── src-tauri/              # Tauri v2 desktop application, system tray monitor, and IPC bridge
├── ui/                     # Modern React 19 + TypeScript + Tailwind CSS interface
├── packaging/              # Distribution packaging scripts (Arch Linux PKGBUILD, desktop entry)
├── docs/                   # Architectural RFCs and technical specifications
└── tasks/                  # Task specifications and acceptance criteria
```

### Key Modules:
- **`crates/portly-core`**: Reusable business logic with zero GUI dependencies. Contains the cross-platform socket scanner, process resolver, CWD discovery, action dispatchers, and persistent configuration.
- **`crates/portly-cli`**: Terminal binary (`portly-cli`) consuming `portly-core`. Exposes subcommands (`list`, `find`, `kill`, `open`, `cwd`, `watch`, `conflicts`, `config`, `completions`, `gui`).
- **`src-tauri`**: Native desktop shell and background system tray daemon connecting frontend commands to `portly-core` via Tauri v2 IPC commands.
- **`ui`**: Single-page application built with React 19, Vite, and Tailwind CSS.

---

## 3. Development Prerequisites

Ensure the following tools are installed on your workstation:

### Base Tooling
- **Rust**: Version 1.80+ (2021 edition) via `rustup` (`rustup default stable`).
- **Node.js**: Version 22+ with `pnpm` (`pnpm@^10`).
- **Git**: Configured with your GPG or SSH signing key.

### Platform-Specific Dependencies

#### Linux (Ubuntu / Debian / Arch / Fedora)
Linux requires GTK3 and WebKit2GTK for Tauri v2:
- **Arch Linux / Omarchy**:
  ```bash
  pkexec pacman -S --needed webkit2gtk-4.1 libappindicator-gtk3 librsvg patchelf
  ```
- **Ubuntu / Debian**:
  ```bash
  sudo apt-get update && sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
  ```
- **Fedora**:
  ```bash
  sudo dnf install -y webkit2gtk4.1-devel libappindicator-gtk3-devel librsvg2-devel patchelf
  ```

#### macOS
- Xcode Command Line Tools (`xcode-select --install`).

#### Windows
- Microsoft Visual Studio C++ Build Tools (MSVC).
- WebView2 runtime (pre-installed on Windows 10/11).

---

## 4. Local Setup & Build Workflow

### 1. Clone Repository & Install Dependencies
```bash
git clone https://github.com/danny/portly.git
cd portly
pnpm install
```

### 2. Run the CLI Locally
```bash
# Run CLI list command
cargo run -p portly-cli -- list

# Run with custom arguments
cargo run -p portly-cli -- find 3000
cargo run -p portly-cli -- kill 3000 --force
```

### 3. Run the Desktop GUI in Development Mode
```bash
# Hot-reloads frontend and rebuilds Rust backend on changes
pnpm tauri dev
```

### 4. Build Production Binaries & Bundles
```bash
# Build standalone release CLI binary
pnpm cli:build
# Or directly via Cargo:
cargo build --release -p portly-cli

# Build full desktop application bundle (.deb, .AppImage, .dmg, or .msi)
pnpm tauri build
```

---

## 5. Core Engineering Principles

All contributions must adhere to these non-negotiable guidelines:

1. **Zero-Mock Rule (Production-Ready by Default)**:
   - Never write mock data or toy implementations.
   - All socket queries must inspect real operating system networking tables (`netstat2`, `/proc`, `libproc`, Win32 IP Helper).
   - All process discovery must inspect real OS process tables and active file systems.

2. **Privilege Escalation Safety**:
   - On Linux, **ALWAYS use `pkexec` instead of `sudo`** when root privileges are required. Never invoke `sudo` in automated tasks as it blocks on terminal password prompts.
   - On Windows, elevate via UAC `runas`. On macOS, use `osascript` or `authopen`.
   - Never terminate processes destructively without user awareness. Terminating a process must default to graceful termination (`SIGTERM`), with fallback or user confirmation for forced termination (`SIGKILL`).

3. **Cross-Platform Compatibility**:
   - Every core feature must account for Linux, macOS, and Windows. If an OS does not support an action (such as specific `/proc` path queries), implement proper platform abstraction traits or `#[cfg(target_os = "...")]` branches.

4. **No Em-Dashes**:
   - Never use em-dashes in prose, comments, commit messages, or documentation. Use standard hyphens (`-`) or colons (`:`).

5. **Architecture Over Shortcut Bias**:
   - Maintain strict layer boundaries. Do not leak Tauri IPC types into `portly-core`, and do not call CLI printing logic inside business libraries.

---

## 6. Git Workflow & Conventional Commits

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for all commits and pull requests.

### Commit Format
```text
<type>(<scope>): <short description in imperative mood>

[optional body explaining context and trade-offs]

[optional footer(s), e.g. Fixes #123]
```

### Allowed Types:
- `feat`: A new user-facing feature.
- `fix`: A bug fix.
- `refactor`: Code change that neither fixes a bug nor adds a feature.
- `test`: Adding or correcting tests.
- `docs`: Documentation changes only.
- `chore`: Tooling, configuration, or dependency updates.

### Allowed Scopes:
- `(core)`: Changes to `crates/portly-core`.
- `(cli)`: Changes to `crates/portly-cli`.
- `(tauri)`: Changes to `src-tauri`.
- `(ui)`: Changes to the frontend in `ui/`.
- `(packaging)`: Packaging scripts, PKGBUILD, or desktop files.
- `(docs)`: Documentation in `docs/` or markdown guides.

### Examples:
- `feat(core): add UDP socket scanning support`
- `fix(cli): correct privilege escalation flag to --escalate`
- `docs: update quickstarts section with config commands`

---

## 7. Verification & Quality Gates

Before opening a pull request or submitting code, you must run and pass the following quality gates:

### Rust Verification
```bash
# 1. Format check
cargo fmt --all -- --check

# 2. Strict linter check
cargo clippy --all-targets --all-features -- -D warnings

# 3. Unit and integration tests
cargo test --all
```

### Frontend Verification
```bash
# 1. Type check
pnpm --filter portly-ui lint

# 2. Production build test
pnpm --filter portly-ui build
```

---

## 8. Pull Request Submission Checklist

When opening a Pull Request:
- [ ] Title follows the Conventional Commits format (e.g. `feat(core): implement TCP conflict detector`).
- [ ] Code is formatted with `cargo fmt --all`.
- [ ] No Clippy warnings (`cargo clippy --all-targets --all-features -- -D warnings`).
- [ ] Frontend builds cleanly with zero TypeScript errors (`pnpm tsc --noEmit`).
- [ ] All tests pass (`cargo test --all`).
- [ ] Documentation and comments are updated where applicable.
- [ ] Verification evidence (test output, CLI snippet, or screenshot for UI changes) is included in the PR description.
- [ ] No em-dashes are used anywhere in the diff or commit messages.

---

## 9. Related Documentation & Specifications

- [Architecture & Internals](docs/ARCHITECTURE.md)
- [IPC Contract Specification](docs/IPC-CONTRACT.md)
- [Privilege Escalation Specification](docs/PRIVILEGE-ESCALATION.md)
- [OS Support Matrix](docs/OS-SUPPORT.md)
- [Project Roadmap](docs/ROADMAP.md)
- [AI Contributor Guidelines](AGENTS.md)
