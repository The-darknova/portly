# Portly Developer Cheatsheet & Agent Instructions

This document provides concise command references and build recipes for Portly.

## Common Commands

### Rust Workspace
```bash
# Build all crates in workspace
cargo build

# Run unit and integration tests
cargo test --all

# Run clippy linter with strict warnings
cargo clippy --all-targets --all-features -- -D warnings

# Format all Rust files
cargo fmt --all
```

### CLI Binary
```bash
# Run CLI list command directly
cargo run -p portly-cli -- list

# Find specific port or process
cargo run -p portly-cli -- find 3000

# Terminate process on a port
cargo run -p portly-cli -- kill 3000

# Open in browser
cargo run -p portly-cli -- open 3000
```

### Desktop UI (Tauri v2 + React 19)
```bash
# Install frontend dependencies
pnpm install

# Start Vite dev server only
pnpm --filter ui dev

# Launch Tauri v2 desktop application in development mode
pnpm tauri dev

# Build production desktop bundle
pnpm tauri build
```

## Rules & Coding Standards
- Never use em-dashes (`—`). Use standard hyphens (`-`) or colons (`:`).
- Always use `pkexec` when running commands requiring root privileges on Linux. Never use `sudo`.
- Zero-mock policy: Always write real OS socket inspection logic.
- Conventional Commits: All commits must follow `type(scope): description` (e.g., `feat(core): implement TCP scanner`).
- Verification First: Always test and provide verifiable evidence before marking tasks complete.
