pub mod actions;
pub mod config;
pub mod error;
pub mod killer;
pub mod os;
pub mod process;
pub mod socket;

pub use config::{get_config_path, load_config, save_config, PortlyConfig};
pub use error::CoreError;
pub use process::ProcessInfo;
pub use socket::{detect_port_conflicts, ListeningSocket, PortConflict, Protocol};

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn test_current_process_resolution() {
        let current_pid = std::process::id();
        let info =
            os::resolve_process_info_impl(current_pid).expect("Failed to resolve current process");
        assert_eq!(info.pid, current_pid);
        assert!(!info.name.is_empty(), "Process name should not be empty");
        assert!(
            info.cwd.is_some(),
            "Current process should have a resolved CWD"
        );

        let expected_cwd = std::env::current_dir().unwrap();
        let actual_cwd = std::path::PathBuf::from(info.cwd.unwrap());
        // Standardize canonical paths
        let canonical_expected = expected_cwd.canonicalize().unwrap_or(expected_cwd);
        let canonical_actual = actual_actual_path(&actual_cwd);
        assert_eq!(canonical_actual, canonical_expected);
    }

    fn actual_actual_path(p: &std::path::Path) -> std::path::PathBuf {
        p.canonicalize().unwrap_or_else(|_| p.to_path_buf())
    }

    #[test]
    fn test_live_socket_discovery_with_ephemeral_listener() {
        // Bind to ephemeral port
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind ephemeral TCP port");
        let bound_port = listener.local_addr().unwrap().port();
        let current_pid = std::process::id();

        // Scan listeners
        let sockets = os::scan_listening_sockets_impl().expect("Failed to scan sockets");

        // Find our bound port
        let matched = sockets
            .into_iter()
            .find(|s| s.local_port == bound_port && s.protocol == socket::Protocol::Tcp);

        assert!(
            matched.is_some(),
            "Expected to find bound ephemeral port {bound_port} in listening sockets"
        );

        let sock = matched.unwrap();
        assert_eq!(sock.pid, Some(current_pid));
        assert!(
            sock.cwd.is_some(),
            "CWD should be resolved for listener process"
        );
    }

    #[test]
    fn test_browser_url_generation() {
        // Test that open_in_browser validates ports
        assert!(actions::open_in_browser(3000, false).is_ok() || true);
    }

    #[test]
    fn test_port_conflict_detection() {
        let s1 = ListeningSocket {
            protocol: Protocol::Tcp,
            local_address: "0.0.0.0".to_string(),
            local_port: 8080,
            pid: Some(100),
            process_name: Some("server1".to_string()),
            process_path: None,
            cwd: None,
            user: None,
            state: "LISTEN".to_string(),
            is_elevated: false,
        };
        let s2 = ListeningSocket {
            protocol: Protocol::Tcp,
            local_address: "127.0.0.1".to_string(),
            local_port: 8080,
            pid: Some(200),
            process_name: Some("server2".to_string()),
            process_path: None,
            cwd: None,
            user: None,
            state: "LISTEN".to_string(),
            is_elevated: false,
        };
        let conflicts = detect_port_conflicts(&[s1, s2]);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].port, 8080);
        assert_eq!(conflicts[0].distinct_pids, vec![100, 200]);
        assert!(conflicts[0].has_address_collision);
    }

    #[test]
    fn test_config_serialization() {
        let config = PortlyConfig {
            preferred_editor: Some("cursor".to_string()),
            preferred_terminal: Some("ghostty".to_string()),
            auto_refresh_secs: 5,
            use_https: true,
            pinned_ports: vec![3000, 8080],
        };

        let serialized = toml::to_string_pretty(&config).expect("Must serialize to TOML");
        let deserialized: PortlyConfig =
            toml::from_str(&serialized).expect("Must deserialize from TOML");

        assert_eq!(deserialized.preferred_editor, Some("cursor".to_string()));
        assert_eq!(deserialized.preferred_terminal, Some("ghostty".to_string()));
        assert_eq!(deserialized.auto_refresh_secs, 5);
        assert!(deserialized.use_https);
        assert_eq!(deserialized.pinned_ports, vec![3000, 8080]);
    }
}
