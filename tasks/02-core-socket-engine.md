# Task 02: Core Socket Discovery Engine

## Objective
Implement socket scanning in `crates/portly-core` to discover all active TCP and UDP listening sockets across IPv4 and IPv6.

## Acceptance Criteria
- [x] Implement `socket.rs` with `ListeningSocket` data structure.
- [x] Discover TCP listening sockets on `0.0.0.0`, `127.0.0.1`, `::`, `::1`, and specific interface IPs.
- [x] Associate sockets with their owning Process ID (PID).
- [x] Filter out non-listening client sockets (unless specifically requested).
- [x] Provide comprehensive unit and live integration tests scanning the current system's real sockets.
