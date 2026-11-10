use portly_core::socket::ListeningSocket;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KillRequest {
    pub pid: u32,
    pub force: bool,
    pub escalate: bool,
}

#[derive(Debug, Deserialize)]
pub struct QuickActionRequest {
    pub port: u16,
    pub protocol: Option<String>,
    pub action: String, // "browser" | "terminal" | "file_manager" | "editor"
    pub target_path: Option<String>,
}

#[tauri::command]
pub async fn scan_ports() -> Result<Vec<ListeningSocket>, String> {
    tokio::task::spawn_blocking(|| {
        portly_core::os::scan_listening_sockets_impl().map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn kill_process(req: KillRequest) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        if req.escalate {
            portly_core::killer::terminate_with_elevation(req.pid, req.force)
                .map_err(|e| e.to_string())
        } else {
            portly_core::killer::terminate_process(req.pid, req.force)
                .or_else(|_| portly_core::killer::terminate_with_elevation(req.pid, req.force))
                .map_err(|e| e.to_string())
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn execute_action(req: QuickActionRequest) -> Result<(), String> {
    tokio::task::spawn_blocking(move || match req.action.as_str() {
        "browser" => {
            let use_https = req.protocol.as_deref() == Some("https");
            portly_core::actions::open_in_browser(req.port, use_https).map_err(|e| e.to_string())
        }
        "terminal" => {
            let path = req.target_path.ok_or("Missing target_path")?;
            portly_core::actions::open_directory_in_terminal(std::path::Path::new(&path))
                .map_err(|e| e.to_string())
        }
        "file_manager" => {
            let path = req.target_path.ok_or("Missing target_path")?;
            portly_core::actions::open_directory_in_file_manager(std::path::Path::new(&path))
                .map_err(|e| e.to_string())
        }
        other => Err(format!("Unknown action: {other}")),
    })
    .await
    .map_err(|e| e.to_string())?
}
