use crate::error::CoreError;
use std::path::Path;
use std::process::Command;

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

pub fn open_directory_in_editor<P: AsRef<Path>>(
    path: P,
    preferred_editor: Option<&str>,
) -> Result<(), CoreError> {
    let p = path.as_ref();
    if !p.exists() {
        return Err(CoreError::ActionFailed(format!(
            "Path does not exist: {}",
            p.display()
        )));
    }

    if let Some(ed) = preferred_editor {
        if which::which(ed).is_ok() && Command::new(ed).arg(p).spawn().is_ok() {
            return Ok(());
        }
    }

    // Default editor hierarchy: cursor -> code -> zed -> subl
    let editors = ["cursor", "code", "zed", "subl"];
    for ed in editors {
        if which::which(ed).is_ok() && Command::new(ed).arg(p).spawn().is_ok() {
            return Ok(());
        }
    }

    Err(CoreError::ActionFailed(
        "No supported code editor found in PATH".to_string(),
    ))
}
