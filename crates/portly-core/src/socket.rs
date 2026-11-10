use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
