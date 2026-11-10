use crate::error::CoreError;
use crate::process::ProcessInfo;
use crate::socket::{ListeningSocket, Protocol};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn scan_listening_sockets_impl() -> Result<Vec<ListeningSocket>, CoreError> {
    use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

    let sockets = get_sockets_info(af_flags, proto_flags)
        .map_err(|e| CoreError::SocketScan(e.to_string()))?;

    let mut result = Vec::new();

    for s in sockets {
        let (protocol, local_addr, local_port, state_str) = match s.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp) => {
                let state_str = format!("{:?}", tcp.state);
                if state_str != "Listen" {
                    continue;
                }
                (
                    Protocol::Tcp,
                    tcp.local_addr.to_string(),
                    tcp.local_port,
                    "LISTEN".to_string(),
                )
            }
            ProtocolSocketInfo::Udp(udp) => (
                Protocol::Udp,
                udp.local_addr.to_string(),
                udp.local_port,
                "LISTEN".to_string(),
            ),
        };

        let pid = s.associated_pids.first().copied();
        let mut proc_name = None;
        let mut proc_path = None;
        let mut cwd = None;
        let mut user = None;
        let mut is_elevated = false;

        if let Some(p) = pid {
            if let Ok(info) = resolve_process_info_impl(p) {
                proc_name = Some(info.name);
                proc_path = info.exe_path;
                cwd = info.cwd;
                user = info.user;
                if user.as_deref() == Some("root") {
                    is_elevated = true;
                }
            }
        } else {
            // PID could not be inspected due to unprivileged access on root-owned socket
            let (known_name, known_user) = identify_well_known_system_service(local_port);
            proc_name = known_name;
            user = known_user;
            is_elevated = true;
        }

        result.push(ListeningSocket {
            protocol,
            local_address: local_addr,
            local_port,
            pid,
            process_name: proc_name,
            process_path: proc_path,
            cwd,
            user,
            state: state_str,
            is_elevated,
        });
    }

    Ok(result)
}

pub fn resolve_process_info_impl(pid: u32) -> Result<ProcessInfo, CoreError> {
    let proc_path = format!("/proc/{pid}");
    if !Path::new(&proc_path).exists() {
        return Err(CoreError::ProcessNotFound(pid));
    }

    let name = fs::read_to_string(format!("{proc_path}/comm"))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| format!("pid-{pid}"));

    let exe_path = fs::read_link(format!("{proc_path}/exe"))
        .ok()
        .map(|p| p.to_string_lossy().to_string());

    let cwd = fs::read_link(format!("{proc_path}/cwd"))
        .ok()
        .map(|p| p.to_string_lossy().to_string());

    let cmdline = fs::read(format!("{proc_path}/cmdline"))
        .ok()
        .map(|bytes| {
            bytes
                .split(|&b| b == 0)
                .filter(|slice| !slice.is_empty())
                .map(|slice| String::from_utf8_lossy(slice).to_string())
                .collect()
        })
        .unwrap_or_default();

    // Resolve owner / user
    let user = resolve_process_username(pid);

    // Resolve memory RSS in bytes from statm
    let memory_bytes = fs::read_to_string(format!("{proc_path}/statm"))
        .ok()
        .and_then(|content| {
            let mut parts = content.split_whitespace();
            parts.next(); // total program size
            parts
                .next()
                .and_then(|rss_pages| rss_pages.parse::<u64>().ok())
        })
        .map(|rss| rss * 4096)
        .unwrap_or(0);

    Ok(ProcessInfo {
        pid,
        name,
        exe_path,
        cmdline,
        cwd,
        user,
        memory_bytes,
        cpu_usage: 0.0,
    })
}

fn resolve_process_username(pid: u32) -> Option<String> {
    let status_path = format!("/proc/{pid}/status");
    let content = fs::read_to_string(status_path).ok()?;
    for line in content.lines() {
        if line.starts_with("Uid:") {
            let mut parts = line.split_whitespace();
            parts.next(); // "Uid:"
            if let Some(uid_str) = parts.next() {
                if let Ok(uid) = uid_str.parse::<u32>() {
                    if uid == 0 {
                        return Some("root".to_string());
                    }
                    let map = build_uid_to_username_map();
                    return map.get(&uid).cloned();
                }
            }
        }
    }
    None
}

fn build_uid_to_username_map() -> HashMap<u32, String> {
    let mut map = HashMap::new();
    if let Ok(content) = fs::read_to_string("/etc/passwd") {
        for line in content.lines() {
            let fields: Vec<&str> = line.split(':').collect();
            if fields.len() >= 3 {
                let username = fields[0];
                if let Ok(uid) = fields[2].parse::<u32>() {
                    map.insert(uid, username.to_string());
                }
            }
        }
    }
    map
}

fn identify_well_known_system_service(port: u16) -> (Option<String>, Option<String>) {
    match port {
        22 => (Some("sshd".to_string()), Some("root".to_string())),
        53 => (
            Some("systemd-resolved".to_string()),
            Some("systemd-resolve".to_string()),
        ),
        67 | 68 => (
            Some("dhcp-client / networkd".to_string()),
            Some("root".to_string()),
        ),
        80 => (
            Some("httpd / nginx / caddy".to_string()),
            Some("www-data".to_string()),
        ),
        443 => (
            Some("https / web server".to_string()),
            Some("www-data".to_string()),
        ),
        631 => (
            Some("cupsd (Printing)".to_string()),
            Some("root".to_string()),
        ),
        5353 => (
            Some("avahi-daemon (mDNS)".to_string()),
            Some("avahi".to_string()),
        ),
        41641 => (
            Some("tailscaled (Tailscale)".to_string()),
            Some("root".to_string()),
        ),
        _ => (None, Some("root/system".to_string())),
    }
}

pub fn terminate_process_impl(pid: u32, force: bool) -> Result<(), CoreError> {
    let sig = if force { "-9" } else { "-15" };
    let status = Command::new("kill")
        .args([sig, &pid.to_string()])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(CoreError::PermissionDenied(pid))
    }
}

pub fn terminate_with_elevation_impl(pid: u32, force: bool) -> Result<(), CoreError> {
    let sig = if force { "-9" } else { "-15" };
    let status = Command::new("pkexec")
        .args(["kill", sig, &pid.to_string()])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(CoreError::TerminationFailed {
            pid,
            reason: "pkexec failed or cancelled by user".to_string(),
        })
    }
}

pub fn open_terminal_in_path_impl(path: &Path) -> Result<(), CoreError> {
    // Detect installed terminals on Linux: ghostty, alacritty, kitty, foot, gnome-terminal, xterm
    let terminals = [
        "ghostty",
        "alacritty",
        "kitty",
        "foot",
        "gnome-terminal",
        "xterm",
    ];
    for term in terminals {
        if which::which(term).is_ok() {
            let res = match term {
                "ghostty" | "alacritty" | "kitty" | "foot" => {
                    Command::new(term).current_dir(path).spawn()
                }
                "gnome-terminal" => Command::new(term)
                    .arg(format!("--working-directory={}", path.display()))
                    .spawn(),
                _ => Command::new(term).current_dir(path).spawn(),
            };
            if res.is_ok() {
                return Ok(());
            }
        }
    }
    Err(CoreError::ActionFailed(
        "No supported terminal emulator found".to_string(),
    ))
}
