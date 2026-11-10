use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeningSocket {
    pub protocol: Protocol,
    pub local_address: String,
    pub local_port: u16,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
    pub process_path: Option<String>,
    pub cwd: Option<String>,
    pub user: Option<String>,
    pub state: String,
    pub is_elevated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConflict {
    pub port: u16,
    pub protocol: Protocol,
    pub sockets: Vec<ListeningSocket>,
    pub distinct_pids: Vec<u32>,
    pub has_address_collision: bool,
}

/// Detects port conflicts: multiple distinct PIDs on the same port, or
/// overlapping wildcard (0.0.0.0 / ::) and specific interface binds.
pub fn detect_port_conflicts(sockets: &[ListeningSocket]) -> Vec<PortConflict> {
    let mut groups: HashMap<(Protocol, u16), Vec<ListeningSocket>> = HashMap::new();
    for s in sockets {
        groups
            .entry((s.protocol, s.local_port))
            .or_default()
            .push(s.clone());
    }

    let mut conflicts = Vec::new();
    for ((protocol, port), sock_list) in groups {
        if sock_list.len() > 1 {
            let pids: HashSet<u32> = sock_list.iter().filter_map(|s| s.pid).collect();
            let addrs: HashSet<&str> = sock_list.iter().map(|s| s.local_address.as_str()).collect();
            let has_wildcard = addrs.contains("0.0.0.0") || addrs.contains("::");
            let has_specific = addrs.iter().any(|&a| a != "0.0.0.0" && a != "::");

            if pids.len() > 1 || (has_wildcard && has_specific) {
                let mut distinct_pids: Vec<u32> = pids.into_iter().collect();
                distinct_pids.sort_unstable();

                conflicts.push(PortConflict {
                    port,
                    protocol,
                    sockets: sock_list,
                    distinct_pids,
                    has_address_collision: has_wildcard && has_specific,
                });
            }
        }
    }

    conflicts.sort_by_key(|c| c.port);
    conflicts
}
