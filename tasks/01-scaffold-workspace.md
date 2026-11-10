# Task 01: Scaffold Cargo & pnpm Workspace

## Objective
Initialize the unified Cargo workspace containing `portly-core`, `portly-cli`, and `src-tauri`, along with the `ui/` React 19 frontend workspace.

## Acceptance Criteria
- [x] Root `Cargo.toml` declares members: `crates/portly-core`, `crates/portly-cli`, `src-tauri`.
- [x] Root `package.json` contains development and build scripts.
- [x] `crates/portly-core` compiles with zero errors or clippy warnings.
- [x] `crates/portly-cli` compiles and exposes a basic `--version` output.
- [x] `src-tauri` is configured with Tauri v2 dependencies and minimal `tauri.conf.json`.
- [x] `ui/` is initialized with Vite, React 19, TypeScript, and Tailwind CSS.
- [x] `pnpm install` and `cargo check` run cleanly.
