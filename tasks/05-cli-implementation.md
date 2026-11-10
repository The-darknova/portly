# Task 05: CLI Implementation & Formatting

## Objective
Implement high-ergonomics command-line interface in `crates/portly-cli` using `clap` v4 with colored tabular output and JSON streaming.

## Acceptance Criteria
- [ ] Subcommand `list`: Pretty-printed table showing Port, Protocol, Process, PID, CWD, and Status.
- [ ] Subcommand `find <query>`: Instant search by port number or binary name substring.
- [ ] Subcommand `kill <port_or_pid>`: Safe termination with `--force` flag.
- [ ] Subcommand `open <port>`: Open listening port in default browser.
- [ ] Subcommand `cwd <port>`: Display or open working directory in terminal.
- [ ] Global flag `--json`: Emits structured JSON for scripting and pipelines.
- [ ] Bash, Zsh, and Fish shell completion generation support.
