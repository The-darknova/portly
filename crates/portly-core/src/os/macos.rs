use crate::error::CoreError;
use crate::process::ProcessInfo;
use crate::socket::{ListeningSocket, Protocol};
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use std::path::Path;
use std::process::Command;
use sysinfo::{Pid, ProcessesToUpdate, System, Users};

pub fn scan_listening_sockets_impl() -> Result<Vec<ListeningSocket>, CoreError> {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

    let sockets = get_sockets_info(af_flags, proto_flags)
        .map_err(|e| CoreError::SocketScan(e.to_string()))?;

    let mut result = Vec::new();
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    let users = Users::new_with_refreshed_list();

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
        let is_elevated = false;

        if let Some(p) = pid {
            let sys_pid = Pid::from_u32(p);
            if let Some(process) = sys.process(sys_pid) {
                proc_name = Some(process.name().to_string_lossy().to_string());
                proc_path = process.exe().map(|e| e.to_string_lossy().to_string());
                cwd = process.cwd().map(|c| c.to_string_lossy().to_string());
                user = process
                    .user_id()
                    .and_then(|u| users.get_user_by_id(u))
                    .map(|u| u.name().to_string());
            }

            // Fallback for CWD via lsof if sysinfo couldn't read sandbox path
            if cwd.is_none() {
                if let Ok(resolved_cwd) = resolve_macos_cwd_via_lsof(p) {
                    cwd = Some(resolved_cwd);
                }
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
            user,
            state: state_str,
            is_elevated,
        });
    }

    Ok(result)
}

pub fn resolve_process_info_impl(pid: u32) -> Result<ProcessInfo, CoreError> {
    let mut sys = System::new();
    let sys_pid = Pid::from_u32(pid);
    sys.refresh_processes(ProcessesToUpdate::Some(&[sys_pid]), true);

    let process = sys
        .process(sys_pid)
        .ok_or(CoreError::ProcessNotFound(pid))?;

    let name = process.name().to_string_lossy().to_string();
    let exe_path = process.exe().map(|e| e.to_string_lossy().to_string());
    let mut cwd = process.cwd().map(|c| c.to_string_lossy().to_string());
    if cwd.is_none() {
        if let Ok(resolved_cwd) = resolve_macos_cwd_via_lsof(pid) {
            cwd = Some(resolved_cwd);
        }
    }

    let cmdline = process
        .cmd()
        .iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect();

    let users = Users::new_with_refreshed_list();
    let user = process
        .user_id()
        .and_then(|u| users.get_user_by_id(u))
        .map(|u| u.name().to_string());

    Ok(ProcessInfo {
        pid,
        name,
        exe_path,
        cmdline,
        cwd,
        user,
        memory_bytes: process.memory(),
        cpu_usage: process.cpu_usage(),
    })
}

fn resolve_macos_cwd_via_lsof(pid: u32) -> Result<String, CoreError> {
    let output = Command::new("lsof")
        .args(["-a", "-p", &pid.to_string(), "-d", "cwd", "-Fn"])
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if let Some(path) = line.strip_prefix('n') {
                if !path.is_empty() {
                    return Ok(path.to_string());
                }
            }
        }
    }
    Err(CoreError::Process(format!(
        "Failed to resolve CWD for PID {pid}"
    )))
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
    let script = format!("do shell script \"kill {sig} {pid}\" with administrator privileges");

    let status = Command::new("osascript").args(["-e", &script]).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(CoreError::TerminationFailed {
            pid,
            reason: "macOS administrator authorization cancelled or failed".to_string(),
        })
    }
}

pub fn open_terminal_in_path_impl(path: &Path) -> Result<(), CoreError> {
    let path_str = path
        .to_str()
        .ok_or_else(|| CoreError::ActionFailed("Invalid path string".to_string()))?;

    // Try iTerm2 first if installed, otherwise default macOS Terminal
    let apps = ["iTerm", "Ghostty", "Alacritty", "Terminal"];
    for app in apps {
        if Command::new("open")
            .args(["-a", app, path_str])
            .spawn()
            .is_ok()
        {
            return Ok(());
        }
    }

    Command::new("open")
        .args(["-a", "Terminal", path_str])
        .spawn()
        .map_err(|e| CoreError::ActionFailed(format!("Failed to open Terminal: {e}")))?;

    Ok(())
}
