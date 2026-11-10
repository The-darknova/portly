use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Socket scan error: {0}")]
    SocketScan(String),

    #[error("Process error: {0}")]
    Process(String),

    #[error("Process not found with PID: {0}")]
    ProcessNotFound(u32),

    #[error("Permission denied when accessing PID {0}. Elevated privileges required.")]
    PermissionDenied(u32),

    #[error("Failed to terminate process {pid}: {reason}")]
    TerminationFailed { pid: u32, reason: String },

    #[error("Quick action failed: {0}")]
    ActionFailed(String),
}
