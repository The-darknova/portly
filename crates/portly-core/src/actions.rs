use crate::error::CoreError;
use std::path::Path;

pub fn open_in_browser(port: u16, use_https: bool) -> Result<(), CoreError> {
    let scheme = if use_https { "https" } else { "http" };
    let url = format!("{scheme}://localhost:{port}");
    open::that(&url)
        .map_err(|e| CoreError::ActionFailed(format!("Failed to open browser URL: {e}")))
}

pub fn open_directory_in_file_manager<P: AsRef<Path>>(path: P) -> Result<(), CoreError> {
    let p = path.as_ref();
    if !p.exists() {
        return Err(CoreError::ActionFailed(format!(
            "Path does not exist: {}",
            p.display()
        )));
    }
    open::that(p).map_err(|e| CoreError::ActionFailed(format!("Failed to reveal directory: {e}")))
}

pub fn open_directory_in_terminal<P: AsRef<Path>>(path: P) -> Result<(), CoreError> {
    crate::os::open_terminal_in_path_impl(path.as_ref())
}
