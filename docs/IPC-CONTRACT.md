# Tauri IPC Contract Specification

This document defines the strongly-typed schema and events between the Rust backend (`src-tauri`) and the React 19 frontend (`ui`).

## Types

```typescript
export type Protocol = "tcp" | "udp";
export type SocketState = "LISTEN" | "ESTABLISHED" | "CLOSE_WAIT" | "TIME_WAIT";

export interface ListeningSocket {
  protocol: Protocol;
  local_address: string;
  local_port: number;
  pid: number | null;
  process_name: string | null;
  process_path: string | null;
  cwd: string | null;
  user: string | null;
  state: SocketState;
  elevated: boolean;
}

export interface KillRequest {
  pid: number;
  force: boolean;
  escalate: boolean;
}

export interface KillResult {
  success: boolean;
  message?: string;
}

export interface QuickActionRequest {
  port: number;
  protocol?: string; // 'http' | 'https'
  action: "browser" | "terminal" | "file_manager" | "editor";
  target_path?: string;
  editor?: "code" | "cursor" | "zed";
}
```

## Tauri Commands (Invocations)

### `scan_ports`
- **Signature**: `scan_ports() -> Result<Vec<ListeningSocket>, String>`
- **Description**: Scans all active listening sockets across IPv4 and IPv6, resolving process metadata and CWD.

### `kill_process`
- **Signature**: `kill_process(req: KillRequest) -> Result<KillResult, String>`
- **Description**: Terminates the specified process. If unprivileged termination fails with permission denied and `escalate: true`, triggers OS elevation.

### `execute_action`
- **Signature**: `execute_action(req: QuickActionRequest) -> Result<(), String>`
- **Description**: Opens default web browser, terminal in CWD, file explorer, or designated editor.

## Tauri Events (Push from Backend)

### `port-conflict-detected`
- **Payload**: `{ port: number, pids: number[] }`
- **Description**: Emitted when multiple processes attempt to bind or collide on identical port resources.

### `ports-updated`
- **Payload**: `ListeningSocket[]`
- **Description**: Emitted periodically or on socket change detection during background monitoring.
