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
    let cwd = process.cwd().map(|c| c.to_string_lossy().to_string());

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

pub fn terminate_process_impl(pid: u32, force: bool) -> Result<(), CoreError> {
    let mut cmd = Command::new("taskkill");
    cmd.args(["/PID", &pid.to_string()]);
    if force {
        cmd.arg("/F");
    }

    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(CoreError::PermissionDenied(pid))
    }
}

pub fn terminate_with_elevation_impl(pid: u32, force: bool) -> Result<(), CoreError> {
    let flag = if force { "/F " } else { "" };
    let args = format!("{flag}/PID {pid}");
    let ps_cmd = format!("Start-Process taskkill -ArgumentList '{args}' -Verb RunAs -Wait");

    let status = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &ps_cmd])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(CoreError::TerminationFailed {
            pid,
            reason: "Windows UAC elevation cancelled or failed".to_string(),
        })
    }
}

pub fn open_terminal_in_path_impl(path: &Path) -> Result<(), CoreError> {
    let path_str = path
        .to_str()
        .ok_or_else(|| CoreError::ActionFailed("Invalid path string".to_string()))?;

    // Try Windows Terminal (wt.exe) first
    if Command::new("wt.exe")
        .args(["-d", path_str])
        .spawn()
        .is_ok()
    {
        return Ok(());
    }

    // Try PowerShell
    if Command::new("powershell.exe")
        .args([
            "-NoExit",
            "-Command",
            &format!("Set-Location -LiteralPath '{path_str}'"),
        ])
        .spawn()
        .is_ok()
    {
        return Ok(());
    }

    // Fallback to CMD
    Command::new("cmd.exe")
        .args(["/K", &format!("cd /d \"{path_str}\"")])
        .spawn()
        .map_err(|e| CoreError::ActionFailed(format!("Failed to spawn command prompt: {e}")))?;

    Ok(())
}
