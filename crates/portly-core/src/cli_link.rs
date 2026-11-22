use crate::error::CoreError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Current installation state of the `portly` CLI command on PATH.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CliLinkState {
    /// Managed by a system package manager or installer (e.g. /usr/bin/portly, MSI machine PATH).
    SystemManaged,
    /// Managed CLI link or PATH entry is present and active on PATH.
    Linked,
    /// Managed CLI link exists, but its parent directory is not currently in PATH.
    LinkedNotInPath,
    /// CLI command is not linked on PATH.
    NotLinked,
}

/// Detailed status of the `portly` CLI command link for GUI and CLI reporting.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CliLinkStatus {
    pub state: CliLinkState,
    pub cli_name: String,
    pub source_executable: String,
    pub link_path: Option<String>,
    pub active_on_path: Option<String>,
    pub in_path: bool,
    pub can_uninstall: bool,
    pub message: String,
}

/// Inspect whether `portly` is currently linked and available on `PATH`.
pub fn get_cli_link_status() -> CliLinkStatus {
    get_cli_link_status_impl()
}

/// Link or register the `portly` CLI binary onto `PATH` for the current platform.
pub fn install_cli_link() -> Result<CliLinkStatus, CoreError> {
    install_cli_link_impl()
}

/// Remove a Portly-managed CLI link or user `PATH` entry.
pub fn uninstall_cli_link() -> Result<CliLinkStatus, CoreError> {
    uninstall_cli_link_impl()
}

/// Check if a directory is included in the current process `PATH`.
pub fn is_dir_in_path(dir: &Path) -> bool {
    let Some(path_var) = std::env::var_os("PATH") else {
        return false;
    };
    is_dir_in_path_env(dir, &path_var)
}

fn is_dir_in_path_env(dir: &Path, path_var: &std::ffi::OsStr) -> bool {
    let target = canonical_or_self(dir);
    std::env::split_paths(path_var).any(|entry| {
        let candidate = canonical_or_self(&entry);
        paths_match(&candidate, &target)
    })
}

fn canonical_or_self(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn paths_match(a: &Path, b: &Path) -> bool {
    if cfg!(target_os = "windows") {
        a.to_string_lossy()
            .trim_end_matches(['\\', '/'])
            .eq_ignore_ascii_case(b.to_string_lossy().trim_end_matches(['\\', '/']))
    } else {
        a == b
    }
}

fn find_active_cli_on_path() -> Option<PathBuf> {
    which::which("portly").ok()
}

#[cfg(unix)]
fn user_local_bin_link() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".local").join("bin").join("portly"))
}

#[cfg(unix)]
fn create_or_replace_symlink(source: &Path, link_path: &Path) -> Result<(), CoreError> {
    if let Some(parent) = link_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    match std::fs::symlink_metadata(link_path) {
        Ok(meta) => {
            if meta.file_type().is_symlink() {
                std::fs::remove_file(link_path)?;
            } else {
                return Err(CoreError::ActionFailed(format!(
                    "Refusing to overwrite existing non-symlink file at {}",
                    link_path.display()
                )));
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(CoreError::Io(e)),
    }

    std::os::unix::fs::symlink(source, link_path)?;
    Ok(())
}

#[cfg(unix)]
fn symlink_points_to_source(link_path: &Path, source: &Path) -> bool {
    let Ok(meta) = std::fs::symlink_metadata(link_path) else {
        return false;
    };
    if !meta.file_type().is_symlink() {
        return false;
    }
    let Ok(raw_target) = std::fs::read_link(link_path) else {
        return false;
    };
    let resolved_target = if raw_target.is_relative() {
        link_path
            .parent()
            .map(|p| p.join(&raw_target))
            .unwrap_or(raw_target)
    } else {
        raw_target
    };
    paths_match(
        &canonical_or_self(&resolved_target),
        &canonical_or_self(source),
    )
}

#[cfg(target_os = "linux")]
fn resolve_source_executable_impl() -> Result<PathBuf, CoreError> {
    if let Some(appimage) = std::env::var_os("APPIMAGE") {
        if !appimage.is_empty() {
            let p = PathBuf::from(appimage);
            if p.exists() {
                return Ok(p);
            }
        }
    }
    Ok(std::env::current_exe()?)
}

#[cfg(target_os = "linux")]
fn get_cli_link_status_impl() -> CliLinkStatus {
    let source = resolve_source_executable_impl().unwrap_or_else(|_| PathBuf::from("portly"));
    let running_appimage = std::env::var_os("APPIMAGE")
        .map(|v| !v.is_empty())
        .unwrap_or(false);
    let active = find_active_cli_on_path();
    let user_link = user_local_bin_link();

    if let Some(ref link_path) = user_link {
        if symlink_points_to_source(link_path, &source) {
            let dir_in_path = link_path.parent().map(is_dir_in_path).unwrap_or(false);
            let active_str = active.as_ref().map(|p| p.display().to_string());
            if dir_in_path {
                return CliLinkStatus {
                    state: CliLinkState::Linked,
                    cli_name: "portly".to_string(),
                    source_executable: source.display().to_string(),
                    link_path: Some(link_path.display().to_string()),
                    active_on_path: active_str,
                    in_path: true,
                    can_uninstall: true,
                    message: format!("Linked at {} and available on PATH.", link_path.display()),
                };
            }
            return CliLinkStatus {
                state: CliLinkState::LinkedNotInPath,
                cli_name: "portly".to_string(),
                source_executable: source.display().to_string(),
                link_path: Some(link_path.display().to_string()),
                active_on_path: active_str,
                in_path: false,
                can_uninstall: true,
                message: format!(
                    "Symlinked at {}, but ~/.local/bin is not in PATH. Add `export PATH=\"$HOME/.local/bin:$PATH\"` to your shell profile.",
                    link_path.display()
                ),
            };
        }
    }

    if let Some(ref active_path) = active {
        let is_system_path = active_path == Path::new("/usr/bin/portly")
            || active_path == Path::new("/bin/portly")
            || active_path == Path::new("/usr/local/bin/portly");
        if is_system_path && !running_appimage {
            return CliLinkStatus {
                state: CliLinkState::SystemManaged,
                cli_name: "portly".to_string(),
                source_executable: source.display().to_string(),
                link_path: Some(active_path.display().to_string()),
                active_on_path: Some(active_path.display().to_string()),
                in_path: true,
                can_uninstall: false,
                message: format!("CLI is installed system-wide at {}.", active_path.display()),
            };
        }
    }

    CliLinkStatus {
        state: CliLinkState::NotLinked,
        cli_name: "portly".to_string(),
        source_executable: source.display().to_string(),
        link_path: user_link.map(|p| p.display().to_string()),
        active_on_path: active.map(|p| p.display().to_string()),
        in_path: false,
        can_uninstall: false,
        message: "CLI is not yet linked into ~/.local/bin/portly.".to_string(),
    }
}

#[cfg(target_os = "linux")]
fn install_cli_link_impl() -> Result<CliLinkStatus, CoreError> {
    let source = resolve_source_executable_impl()?;
    let link_path = user_local_bin_link().ok_or_else(|| {
        CoreError::ActionFailed("Could not determine user home directory".to_string())
    })?;
    create_or_replace_symlink(&source, &link_path)?;
    Ok(get_cli_link_status_impl())
}

#[cfg(target_os = "linux")]
fn uninstall_cli_link_impl() -> Result<CliLinkStatus, CoreError> {
    if let Some(link_path) = user_local_bin_link() {
        if let Ok(meta) = std::fs::symlink_metadata(&link_path) {
            if meta.file_type().is_symlink() {
                std::fs::remove_file(&link_path)?;
            }
        }
    }
    Ok(get_cli_link_status_impl())
}

#[cfg(target_os = "macos")]
fn resolve_source_executable_impl() -> Result<PathBuf, CoreError> {
    Ok(std::env::current_exe()?)
}

#[cfg(target_os = "macos")]
fn macos_system_link() -> PathBuf {
    PathBuf::from("/usr/local/bin/portly")
}

#[cfg(target_os = "macos")]
fn shell_single_quote(input: &str) -> String {
    format!("'{}'", input.replace('\'', "'\"'\"'"))
}

#[cfg(target_os = "macos")]
fn run_macos_admin_shell(shell_cmd: &str) -> Result<(), CoreError> {
    let escaped = shell_cmd.replace('\\', "\\\\").replace('"', "\\\"");
    let script = format!("do shell script \"{escaped}\" with administrator privileges");
    let status = std::process::Command::new("osascript")
        .args(["-e", &script])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(CoreError::ActionFailed(
            "macOS administrator authorization cancelled or failed".to_string(),
        ))
    }
}

#[cfg(target_os = "macos")]
fn get_cli_link_status_impl() -> CliLinkStatus {
    let source = resolve_source_executable_impl().unwrap_or_else(|_| PathBuf::from("portly"));
    let active = find_active_cli_on_path();
    let system_link = macos_system_link();
    let user_link = user_local_bin_link();

    for candidate in [Some(system_link.clone()), user_link.clone()]
        .into_iter()
        .flatten()
    {
        if symlink_points_to_source(&candidate, &source) {
            let dir_in_path = candidate.parent().map(is_dir_in_path).unwrap_or(false);
            let active_str = active.as_ref().map(|p| p.display().to_string());
            if dir_in_path {
                return CliLinkStatus {
                    state: CliLinkState::Linked,
                    cli_name: "portly".to_string(),
                    source_executable: source.display().to_string(),
                    link_path: Some(candidate.display().to_string()),
                    active_on_path: active_str,
                    in_path: true,
                    can_uninstall: true,
                    message: format!("Linked at {} and available on PATH.", candidate.display()),
                };
            }
            return CliLinkStatus {
                state: CliLinkState::LinkedNotInPath,
                cli_name: "portly".to_string(),
                source_executable: source.display().to_string(),
                link_path: Some(candidate.display().to_string()),
                active_on_path: active_str,
                in_path: false,
                can_uninstall: true,
                message: format!(
                    "Symlinked at {}, but its directory is not in PATH.",
                    candidate.display()
                ),
            };
        }
    }

    if let Some(ref active_path) = active {
        return CliLinkStatus {
            state: CliLinkState::SystemManaged,
            cli_name: "portly".to_string(),
            source_executable: source.display().to_string(),
            link_path: Some(active_path.display().to_string()),
            active_on_path: Some(active_path.display().to_string()),
            in_path: true,
            can_uninstall: false,
            message: format!("CLI is active on PATH at {}.", active_path.display()),
        };
    }

    CliLinkStatus {
        state: CliLinkState::NotLinked,
        cli_name: "portly".to_string(),
        source_executable: source.display().to_string(),
        link_path: Some(system_link.display().to_string()),
        active_on_path: None,
        in_path: false,
        can_uninstall: false,
        message: "CLI is not linked in /usr/local/bin/portly.".to_string(),
    }
}

#[cfg(target_os = "macos")]
fn install_cli_link_impl() -> Result<CliLinkStatus, CoreError> {
    let source = resolve_source_executable_impl()?;
    let system_link = macos_system_link();

    match create_or_replace_symlink(&source, &system_link) {
        Ok(()) => Ok(get_cli_link_status_impl()),
        Err(CoreError::Io(ref e)) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            if let Ok(meta) = std::fs::symlink_metadata(&system_link) {
                if !meta.file_type().is_symlink() {
                    return Err(CoreError::ActionFailed(format!(
                        "Refusing to overwrite existing non-symlink file at {}",
                        system_link.display()
                    )));
                }
            }
            let cmd = format!(
                "mkdir -p /usr/local/bin && ln -sf {} {}",
                shell_single_quote(&source.to_string_lossy()),
                shell_single_quote(&system_link.to_string_lossy())
            );
            run_macos_admin_shell(&cmd)?;
            Ok(get_cli_link_status_impl())
        }
        Err(e) => Err(e),
    }
}

#[cfg(target_os = "macos")]
fn uninstall_cli_link_impl() -> Result<CliLinkStatus, CoreError> {
    let system_link = macos_system_link();
    if let Ok(meta) = std::fs::symlink_metadata(&system_link) {
        if meta.file_type().is_symlink() {
            match std::fs::remove_file(&system_link) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                    let cmd = format!(
                        "rm -f {}",
                        shell_single_quote(&system_link.to_string_lossy())
                    );
                    run_macos_admin_shell(&cmd)?;
                }
                Err(e) => return Err(CoreError::Io(e)),
            }
        }
    }

    if let Some(user_link) = user_local_bin_link() {
        if let Ok(meta) = std::fs::symlink_metadata(&user_link) {
            if meta.file_type().is_symlink() {
                std::fs::remove_file(&user_link)?;
            }
        }
    }

    Ok(get_cli_link_status_impl())
}

#[cfg(target_os = "windows")]
fn resolve_source_executable_impl() -> Result<PathBuf, CoreError> {
    let exe = std::env::current_exe()?;
    if let Some(parent) = exe.parent() {
        let bundled_cli = parent.join("bin").join("portly.exe");
        if bundled_cli.exists() {
            return Ok(bundled_cli);
        }
    }
    Ok(exe)
}

#[cfg(target_os = "windows")]
fn run_windows_powershell(script: &str, env_dir: Option<&Path>) -> Result<String, CoreError> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    let mut cmd = std::process::Command::new("powershell.exe");
    cmd.creation_flags(CREATE_NO_WINDOW).args([
        "-NoProfile",
        "-NonInteractive",
        "-Command",
        script,
    ]);
    if let Some(dir) = env_dir {
        cmd.env("PORTLY_CLI_DIR", dir);
    }

    let output = cmd.output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(CoreError::ActionFailed(format!(
            "Failed to update Windows User PATH: {stderr}"
        )))
    }
}

#[cfg(target_os = "windows")]
fn read_windows_user_path() -> String {
    run_windows_powershell(
        "[Environment]::GetEnvironmentVariable('Path', 'User')",
        None,
    )
    .unwrap_or_default()
}

#[cfg(target_os = "windows")]
fn get_cli_link_status_impl() -> CliLinkStatus {
    let source = resolve_source_executable_impl().unwrap_or_else(|_| PathBuf::from("portly.exe"));
    let cli_dir = source
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let cli_dir_str = cli_dir.to_string_lossy().to_string();
    let active = find_active_cli_on_path();
    let user_path_raw = read_windows_user_path();
    let in_user_path = windows_path_contains(&user_path_raw, &cli_dir_str);
    let in_proc_path = is_dir_in_path(&cli_dir) || active.is_some();

    if in_user_path {
        return CliLinkStatus {
            state: CliLinkState::Linked,
            cli_name: "portly".to_string(),
            source_executable: source.display().to_string(),
            link_path: Some(cli_dir_str.clone()),
            active_on_path: active.map(|p| p.display().to_string()),
            in_path: true,
            can_uninstall: true,
            message: format!(
                "CLI directory {} is registered in your User PATH.",
                cli_dir_str
            ),
        };
    }

    if in_proc_path {
        let active_str = active.as_ref().map(|p| p.display().to_string());
        return CliLinkStatus {
            state: CliLinkState::SystemManaged,
            cli_name: "portly".to_string(),
            source_executable: source.display().to_string(),
            link_path: Some(cli_dir_str),
            active_on_path: active_str.clone(),
            in_path: true,
            can_uninstall: false,
            message: format!(
                "CLI is active on system PATH ({}).",
                active_str.unwrap_or_else(|| "system PATH".to_string())
            ),
        };
    }

    CliLinkStatus {
        state: CliLinkState::NotLinked,
        cli_name: "portly".to_string(),
        source_executable: source.display().to_string(),
        link_path: Some(cli_dir_str),
        active_on_path: None,
        in_path: false,
        can_uninstall: false,
        message: "CLI directory is not registered in PATH.".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn install_cli_link_impl() -> Result<CliLinkStatus, CoreError> {
    let source = resolve_source_executable_impl()?;
    let cli_dir = source.parent().ok_or_else(|| {
        CoreError::ActionFailed("Could not resolve CLI binary parent directory".to_string())
    })?;
    let current_user_path = read_windows_user_path();
    let updated = append_windows_path(&current_user_path, &cli_dir.to_string_lossy());
    if updated != current_user_path {
        run_windows_powershell(
            "$target = $env:PORTLY_CLI_DIR; \
             $cur = [Environment]::GetEnvironmentVariable('Path', 'User'); \
             $parts = @(); \
             if ($cur) { $parts = $cur -split ';' | Where-Object { $_ -and $_.TrimEnd('\\','/') -ne $target.TrimEnd('\\','/') } }; \
             $parts += $target; \
             [Environment]::SetEnvironmentVariable('Path', ($parts -join ';'), 'User')",
            Some(cli_dir),
        )?;
    }
    Ok(get_cli_link_status_impl())
}

#[cfg(target_os = "windows")]
fn uninstall_cli_link_impl() -> Result<CliLinkStatus, CoreError> {
    let source = resolve_source_executable_impl()?;
    let cli_dir = source.parent().ok_or_else(|| {
        CoreError::ActionFailed("Could not resolve CLI binary parent directory".to_string())
    })?;
    let current_user_path = read_windows_user_path();
    if windows_path_contains(&current_user_path, &cli_dir.to_string_lossy()) {
        run_windows_powershell(
            "$target = $env:PORTLY_CLI_DIR; \
             $cur = [Environment]::GetEnvironmentVariable('Path', 'User'); \
             if ($cur) { \
               $parts = $cur -split ';' | Where-Object { $_ -and $_.TrimEnd('\\','/') -ne $target.TrimEnd('\\','/') }; \
               [Environment]::SetEnvironmentVariable('Path', ($parts -join ';'), 'User') \
             }",
            Some(cli_dir),
        )?;
    }
    Ok(get_cli_link_status_impl())
}

/// Check if a semicolon-delimited Windows PATH string contains `dir` (case-insensitive).
pub fn windows_path_contains(raw_path: &str, dir: &str) -> bool {
    let norm_target = dir.trim().trim_end_matches(['\\', '/']);
    if norm_target.is_empty() {
        return false;
    }
    raw_path.split(';').any(|entry| {
        entry
            .trim()
            .trim_end_matches(['\\', '/'])
            .eq_ignore_ascii_case(norm_target)
    })
}

/// Append `dir` to a semicolon-delimited Windows PATH string if not already present.
pub fn append_windows_path(raw_path: &str, dir: &str) -> String {
    if windows_path_contains(raw_path, dir) {
        return raw_path.to_string();
    }
    let trimmed_dir = dir.trim();
    let mut entries: Vec<&str> = raw_path
        .split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    entries.push(trimmed_dir);
    entries.join(";")
}

/// Remove `dir` from a semicolon-delimited Windows PATH string (case-insensitive).
pub fn remove_windows_path(raw_path: &str, dir: &str) -> String {
    let norm_target = dir.trim().trim_end_matches(['\\', '/']);
    let entries: Vec<&str> = raw_path
        .split(';')
        .map(str::trim)
        .filter(|s| {
            !s.is_empty()
                && !s
                    .trim_end_matches(['\\', '/'])
                    .eq_ignore_ascii_case(norm_target)
        })
        .collect();
    entries.join(";")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows_path_helpers() {
        let base = r"C:\Windows\System32;C:\Users\Alice\bin";
        assert!(!windows_path_contains(base, r"C:\Program Files\Portly\bin"));

        let added = append_windows_path(base, r"C:\Program Files\Portly\bin");
        assert_eq!(
            added,
            r"C:\Windows\System32;C:\Users\Alice\bin;C:\Program Files\Portly\bin"
        );
        assert!(windows_path_contains(
            &added,
            r"c:\program files\portly\bin\"
        ));

        // Idempotent append
        assert_eq!(
            append_windows_path(&added, r"C:\Program Files\Portly\bin\"),
            added
        );

        let removed = remove_windows_path(&added, r"C:\Program Files\Portly\bin");
        assert_eq!(removed, base);
    }

    #[test]
    fn test_is_dir_in_path_env() {
        let tmp = std::env::temp_dir();
        let joined = std::env::join_paths([Path::new("/nonexistent/a"), tmp.as_path()]).unwrap();
        assert!(is_dir_in_path_env(&tmp, &joined));
        assert!(!is_dir_in_path_env(
            Path::new("/definitely/missing/portly-test-bin"),
            &joined
        ));
    }

    #[cfg(unix)]
    #[test]
    fn test_unix_symlink_create_and_verify() {
        let base =
            std::env::temp_dir().join(format!("portly-cli-link-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();

        let source = base.join("source-bin");
        std::fs::write(&source, b"binary").unwrap();
        let link = base.join("bin").join("portly");

        create_or_replace_symlink(&source, &link).unwrap();
        assert!(symlink_points_to_source(&link, &source));

        let _ = std::fs::remove_dir_all(&base);
    }
}
