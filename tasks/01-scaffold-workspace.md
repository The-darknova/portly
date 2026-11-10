# Task 01: Scaffold Cargo & pnpm Workspace

## Objective
Initialize the unified Cargo workspace containing `portly-core`, `portly-cli`, and `src-tauri`, along with the `ui/` React 19 frontend workspace.

## Acceptance Criteria
- [ ] Root `Cargo.toml` declares members: `crates/portly-core`, `crates/portly-cli`, `src-tauri`.
- [ ] Root `package.json` contains development and build scripts.
- [ ] `crates/portly-core` compiles with zero errors or clippy warnings.
- [ ] `crates/portly-cli` compiles and exposes a basic `--version` output.
- [ ] `src-tauri` is configured with Tauri v2 dependencies and minimal `tauri.conf.json`.
- [ ] `ui/` is initialized with Vite, React 19, TypeScript, and Tailwind CSS.
- [ ] `pnpm install` and `cargo check` run cleanly.
