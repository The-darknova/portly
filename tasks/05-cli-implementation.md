# Task 05: CLI Implementation & Formatting

## Objective
Implement high-ergonomics command-line interface in `crates/portly-cli` using `clap` v4 with colored tabular output and JSON streaming.

## Acceptance Criteria
- [x] Subcommand `list`: Pretty-printed table showing Port, Protocol, Process, PID, User, CWD, and Address.
- [x] Subcommand `find <query>`: Instant search by port number or binary name substring.
- [x] Subcommand `kill <port>`: Safe termination with `--force` and `--escalate` flags.
- [x] Subcommand `open <port>`: Open listening port in default browser.
- [x] Subcommand `cwd <port>`: Display or open working directory in terminal, editor, or file manager.
- [x] Subcommand `watch`: Live terminal monitoring with customizable polling interval.
- [x] Subcommand `conflicts`: Detects and reports colliding ports and addresses.
- [x] Subcommand `completions`: Generates tab completion scripts for bash, zsh, fish, powershell, and elvish.
- [x] Global flag `--json`: Emits structured JSON for scripting and pipelines.
