use crate::error::CoreError;
use crate::process::ProcessInfo;
use crate::socket::{ListeningSocket, Protocol};
use std::path::Path;
use std::process::Command;

pub fn scan_listening_sockets_impl() -> Result<Vec<ListeningSocket>, CoreError> {
    // Basic initial scanner using netstat2
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
        let is_elevated = false;

        if let Some(p) = pid {
            if let Ok(info) = resolve_process_info_impl(p) {
                proc_name = Some(info.name);
                proc_path = info.exe_path;
                cwd = info.cwd;
            }
        }

        result.push(ListeningSocket {
            protocol,
            local_address: local_addr,
            local_port,
            pid,
            process_name: proc_name,
            process_path: proc_path,
            cwd,
            user: None,
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

    let name = std::fs::read_to_string(format!("{proc_path}/comm"))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| format!("pid-{pid}"));

    let exe_path = std::fs::read_link(format!("{proc_path}/exe"))
        .ok()
        .map(|p| p.to_string_lossy().to_string());

    let cwd = std::fs::read_link(format!("{proc_path}/cwd"))
        .ok()
        .map(|p| p.to_string_lossy().to_string());

    let cmdline = std::fs::read(format!("{proc_path}/cmdline"))
        .ok()
        .map(|bytes| {
            bytes
                .split(|&b| b == 0)
                .filter(|slice| !slice.is_empty())
                .map(|slice| String::from_utf8_lossy(slice).to_string())
                .collect()
        })
        .unwrap_or_default();

    Ok(ProcessInfo {
        pid,
        name,
        exe_path,
        cmdline,
        cwd,
        user: None,
        memory_bytes: 0,
        cpu_usage: 0.0,
    })
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
    // Detect installed terminals on Linux: ghostty, alacritty, kitty, gnome-terminal, xterm
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
