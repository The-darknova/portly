use crate::error::CoreError;
use crate::process::ProcessInfo;
use crate::socket::ListeningSocket;
use std::path::Path;

pub fn scan_listening_sockets_impl() -> Result<Vec<ListeningSocket>, CoreError> {
    Err(CoreError::SocketScan(
        "macOS scanning not yet implemented on this platform".to_string(),
    ))
}

pub fn resolve_process_info_impl(_pid: u32) -> Result<ProcessInfo, CoreError> {
    Err(CoreError::Process(
        "macOS process resolution not yet implemented".to_string(),
    ))
}

pub fn terminate_process_impl(_pid: u32, _force: bool) -> Result<(), CoreError> {
    Err(CoreError::Process(
        "macOS termination not yet implemented".to_string(),
    ))
}

pub fn terminate_with_elevation_impl(_pid: u32, _force: bool) -> Result<(), CoreError> {
    Err(CoreError::Process(
        "macOS elevated termination not yet implemented".to_string(),
    ))
}

pub fn open_terminal_in_path_impl(_path: &Path) -> Result<(), CoreError> {
    Err(CoreError::ActionFailed(
        "macOS terminal launch not yet implemented".to_string(),
    ))
}
