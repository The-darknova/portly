# Task 02: Core Socket Discovery Engine

## Objective
Implement socket scanning in `crates/portly-core` to discover all active TCP and UDP listening sockets across IPv4 and IPv6.

## Acceptance Criteria
- [ ] Implement `socket.rs` with `ListeningSocket` data structure.
- [ ] Discover TCP listening sockets on `0.0.0.0`, `127.0.0.1`, `::`, `::1`, and specific interface IPs.
- [ ] Associate sockets with their owning Process ID (PID).
- [ ] Filter out non-listening client sockets (unless specifically requested).
- [ ] Provide comprehensive unit and live integration tests scanning the current system's real sockets.
