export type Protocol = "tcp" | "udp";

export interface ListeningSocket {
  protocol: Protocol;
  local_address: string;
  local_port: number;
  pid: number | null;
  process_name: string | null;
  process_path: string | null;
  cwd: string | null;
  user: string | null;
  state: string;
  is_elevated: boolean;
}

export interface PortConflict {
  port: number;
  protocol: Protocol;
  sockets: ListeningSocket[];
  distinct_pids: number[];
  has_address_collision: boolean;
}

export interface KillRequest {
  pid: number;
  force: boolean;
  escalate: boolean;
}

export interface QuickActionRequest {
  port: number;
  protocol?: string;
  action: "browser" | "terminal" | "file_manager" | "editor";
  target_path?: string;
}

export interface PortlyConfig {
  preferred_editor?: string | null;
  preferred_terminal?: string | null;
  auto_refresh_secs: number;
  use_https: boolean;
  pinned_ports: number[];
}
