use clap::{CommandFactory, Parser, Subcommand};
use colored::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, Row, Table};
use portly_core::socket::ListeningSocket;
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(name = "portly", author, version, about = "Cross-platform port and process manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Emit output in JSON format
    #[arg(short, long, global = true)]
    pub json: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// List all active listening ports
    List {
        /// Filter by protocol (tcp or udp)
        #[arg(short, long)]
        proto: Option<String>,
    },
    /// Continuously watch active listening ports in the terminal
    Watch {
        /// Polling interval in seconds (default: 2)
        #[arg(short, long, default_value = "2")]
        interval: u64,

        /// Filter by protocol (tcp or udp)
        #[arg(short, long)]
        proto: Option<String>,
    },
    /// Find a listening port or process by number or name
    Find {
        /// Port number or process name query
        target: String,
    },
    /// Terminate the process listening on a specified port
    Kill {
        /// Port number to terminate
        port: u16,

        /// Force termination (SIGKILL)
        #[arg(short, long)]
        force: bool,

        /// Use privilege escalation (pkexec / UAC) if needed
        #[arg(short = 'e', long)]
        escalate: bool,
    },
    /// Open the listening service in default browser
    Open {
        /// Port number to open (optional: auto-detects active web services if omitted)
        port: Option<u16>,

        /// Use HTTPS instead of HTTP
        #[arg(long)]
        https: bool,
    },
    /// Display or open working directory of process listening on port
    Cwd {
        /// Port number
        port: u16,

        /// Open working directory in an external terminal
        #[arg(short, long)]
        terminal: bool,

        /// Open working directory in preferred code editor (cursor, code, zed)
        #[arg(short, long)]
        editor: bool,

        /// Open working directory in file manager
        #[arg(short, long)]
        reveal: bool,
    },
    /// Generate shell auto-completion script
    Completions {
        /// Target shell (bash, zsh, fish, powershell, elvish)
        shell: clap_complete::Shell,
    },
    /// Detect and report conflicting or colliding listening ports
    Conflicts,
    /// View or update Portly persistent configuration
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
    },
    /// Launch the desktop GUI
    Gui,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigAction {
    /// Show current configuration values
    Show,
    /// Print the filesystem path to config.toml
    Path,
    /// Set preferred code editor (cursor, code, zed, subl)
    SetEditor { editor: String },
    /// Set preferred terminal emulator (ghostty, kitty, alacritty, wt.exe)
    SetTerminal { terminal: String },
    /// Set default polling refresh interval in seconds
    SetRefresh { interval: u64 },
    /// Set default HTTPS usage (true/false)
    SetHttps { enabled: bool },
    /// Pin a port to the top of the list
    Pin { port: u16 },
    /// Unpin a port
    Unpin { port: u16 },
}

pub fn execute_cli_command(cli: Cli) {
    match cli.command.unwrap_or(Commands::List { proto: None }) {
        Commands::List { proto } => {
            handle_list(cli.json, proto);
        }
        Commands::Watch { interval, proto } => {
            handle_watch(interval, proto);
        }
        Commands::Find { target } => {
            handle_find(cli.json, &target);
        }
        Commands::Kill {
            port,
            force,
            escalate,
        } => {
            handle_kill(port, force, escalate);
        }
        Commands::Open { port, https } => {
            handle_open(port, https);
        }
        Commands::Cwd {
            port,
            terminal,
            editor,
            reveal,
        } => {
            handle_cwd(port, terminal, editor, reveal);
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "portly", &mut std::io::stdout());
        }
        Commands::Conflicts => {
            handle_conflicts(cli.json);
        }
        Commands::Config { action } => {
            handle_config(action, cli.json);
        }
        Commands::Gui => {
            println!("Launching Portly Desktop GUI...");
        }
    }
}

pub fn handle_list(json: bool, proto_filter: Option<String>) {
    let sockets = match portly_core::os::scan_listening_sockets_impl() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", "Error scanning sockets".red().bold(), e);
            std::process::exit(1);
        }
    };

    let filtered: Vec<ListeningSocket> = sockets
        .into_iter()
        .filter(|s| {
            if let Some(ref p) = proto_filter {
                let p_str = format!("{:?}", s.protocol).to_lowercase();
                p_str == p.to_lowercase()
            } else {
                true
            }
        })
        .collect();

    if json {
        println!("{}", serde_json::to_string_pretty(&filtered).unwrap());
    } else {
        render_table(&filtered);
    }
}

pub fn handle_watch(interval_secs: u64, proto_filter: Option<String>) {
    println!("{}", "Watching listening ports (Ctrl+C to stop)...".cyan());
    loop {
        // Clear terminal screen using ANSI escape sequence
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "{} - Refreshing every {}s (Ctrl+C to quit)\n",
            "Portly Live Monitor".green().bold(),
            interval_secs
        );
        handle_list(false, proto_filter.clone());
        sleep(Duration::from_secs(interval_secs.max(1)));
    }
}

pub fn handle_find(json: bool, target: &str) {
    let sockets = match portly_core::os::scan_listening_sockets_impl() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", "Error scanning sockets".red().bold(), e);
            std::process::exit(1);
        }
    };

    let matches: Vec<ListeningSocket> = sockets
        .into_iter()
        .filter(|s| {
            s.local_port.to_string() == target
                || s.process_name
                    .as_deref()
                    .map(|n| n.to_lowercase().contains(&target.to_lowercase()))
                    .unwrap_or(false)
                || s.pid.map(|p| p.to_string() == target).unwrap_or(false)
        })
        .collect();

    if json {
        println!("{}", serde_json::to_string_pretty(&matches).unwrap());
    } else if matches.is_empty() {
        println!(
            "{}",
            format!("No listening sockets found matching '{target}'").yellow()
        );
    } else {
        render_table(&matches);
    }
}

pub fn handle_conflicts(json: bool) {
    let sockets = match portly_core::os::scan_listening_sockets_impl() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", "Error scanning sockets".red().bold(), e);
            std::process::exit(1);
        }
    };

    let conflicts = portly_core::detect_port_conflicts(&sockets);

    if json {
        println!("{}", serde_json::to_string_pretty(&conflicts).unwrap());
    } else if conflicts.is_empty() {
        println!(
            "{}",
            "✓ No port conflicts detected across active sockets."
                .green()
                .bold()
        );
    } else {
        println!(
            "{}: Found {} port conflict(s) across active sockets!\n",
            "WARNING".red().bold(),
            conflicts.len()
        );
        for c in &conflicts {
            println!(
                "• Port {} ({:?}) - Competing PIDs: {:?}",
                c.port.to_string().yellow().bold(),
                c.protocol,
                c.distinct_pids
            );
            for s in &c.sockets {
                println!(
                    "    - Address: {:<15} | Process: {:<15} | PID: {:?}",
                    s.local_address,
                    s.process_name.as_deref().unwrap_or("-"),
                    s.pid
                );
            }
        }
    }
}

pub fn handle_kill(port: u16, force: bool, escalate: bool) {
    let sockets = match portly_core::os::scan_listening_sockets_impl() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", "Error scanning sockets".red().bold(), e);
            std::process::exit(1);
        }
    };

    let target_sock = sockets.into_iter().find(|s| s.local_port == port);

    let Some(sock) = target_sock else {
        eprintln!(
            "{}: No process found listening on port {}",
            "Error".red().bold(),
            port
        );
        std::process::exit(1);
    };

    let Some(pid) = sock.pid else {
        eprintln!(
            "{}: Found port {} but could not determine PID (elevated process?)",
            "Error".red().bold(),
            port
        );
        std::process::exit(1);
    };

    println!(
        "Terminating process {} (PID: {}) on port {}...",
        sock.process_name.as_deref().unwrap_or("unknown").bold(),
        pid.to_string().cyan(),
        port.to_string().green()
    );

    let res = if escalate {
        portly_core::killer::terminate_with_elevation(pid, force)
    } else {
        portly_core::killer::terminate_process(pid, force).or_else(|_| {
            println!("Unprivileged termination failed. Retrying with elevation...");
            portly_core::killer::terminate_with_elevation(pid, force)
        })
    };

    match res {
        Ok(_) => println!("{}", "Process terminated successfully.".green().bold()),
        Err(e) => {
            eprintln!("{}: {}", "Failed to terminate process".red().bold(), e);
            std::process::exit(1);
        }
    }
}

pub fn handle_open(port: Option<u16>, https: bool) {
    let target_port = match port {
        Some(p) => p,
        None => {
            let sockets = portly_core::os::scan_listening_sockets_impl().unwrap_or_default();
            let web_ports = [3000, 5173, 8080, 8000, 4200, 8081, 8888, 80, 443];
            let detected_web = sockets.iter().find(|s| web_ports.contains(&s.local_port));

            if let Some(sock) = detected_web {
                println!(
                    "No port specified. Detected active web service on port {} ({})",
                    sock.local_port.to_string().cyan().bold(),
                    sock.process_name.as_deref().unwrap_or("unknown")
                );
                sock.local_port
            } else if let Some(first_sock) = sockets.iter().find(|s| s.local_port > 1024) {
                println!(
                    "No port specified. Detected active service on port {} ({})",
                    first_sock.local_port.to_string().cyan().bold(),
                    first_sock.process_name.as_deref().unwrap_or("unknown")
                );
                first_sock.local_port
            } else {
                eprintln!(
                    "{}: No active listening web ports found. Specify a port: 'portly open <PORT>'",
                    "Notice".yellow().bold()
                );
                return;
            }
        }
    };

    if let Ok(sockets) = portly_core::os::scan_listening_sockets_impl() {
        let is_listening = sockets.iter().any(|s| s.local_port == target_port);
        if !is_listening {
            println!(
                "{}: No active process detected listening on port {}. If your server is not running, the browser may report 'Connection refused'.",
                "Note".yellow().bold(),
                target_port.to_string().yellow()
            );
        }
    }

    println!(
        "Opening port {} in default browser...",
        target_port.to_string().green().bold()
    );
    if let Err(e) = portly_core::actions::open_in_browser(target_port, https) {
        eprintln!("{}: {}", "Failed to open browser".red().bold(), e);
        std::process::exit(1);
    }
}

pub fn handle_cwd(port: u16, terminal: bool, editor: bool, reveal: bool) {
    let sockets = match portly_core::os::scan_listening_sockets_impl() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", "Error scanning sockets".red().bold(), e);
            std::process::exit(1);
        }
    };

    let target_sock = sockets.into_iter().find(|s| s.local_port == port);

    let Some(sock) = target_sock else {
        eprintln!(
            "{}: No process found listening on port {}",
            "Error".red().bold(),
            port
        );
        std::process::exit(1);
    };

    let Some(cwd) = sock.cwd else {
        eprintln!(
            "{}: Could not determine working directory for port {}",
            "Error".red().bold(),
            port
        );
        std::process::exit(1);
    };

    println!("Working directory: {}", cwd.cyan().bold());

    if terminal {
        if let Err(e) = portly_core::actions::open_directory_in_terminal(&cwd) {
            eprintln!("{}: {}", "Failed to open terminal".red().bold(), e);
        }
    }

    if editor {
        if let Err(e) = portly_core::actions::open_directory_in_editor(&cwd, None) {
            eprintln!("{}: {}", "Failed to open code editor".red().bold(), e);
        }
    }

    if reveal {
        if let Err(e) = portly_core::actions::open_directory_in_file_manager(&cwd) {
            eprintln!("{}: {}", "Failed to reveal directory".red().bold(), e);
        }
    }
}

pub fn render_table(sockets: &[ListeningSocket]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Proto").fg(Color::Cyan),
            Cell::new("Port").fg(Color::Green),
            Cell::new("Process").fg(Color::Yellow),
            Cell::new("PID").fg(Color::Magenta),
            Cell::new("User").fg(Color::White),
            Cell::new("Working Directory (CWD)").fg(Color::Blue),
            Cell::new("Address"),
        ]);

    for s in sockets {
        let proto = format!("{:?}", s.protocol).to_uppercase();
        let port = s.local_port.to_string();
        let name = s.process_name.as_deref().unwrap_or("-");
        let pid = s
            .pid
            .map(|p| p.to_string())
            .unwrap_or_else(|| "-".to_string());
        let user = s.user.as_deref().unwrap_or("-");
        let cwd = s.cwd.as_deref().unwrap_or("-");
        let addr = &s.local_address;

        let user_cell = if user == "root" || user == "root/system" {
            Cell::new(user).fg(Color::Red)
        } else {
            Cell::new(user).fg(Color::White)
        };

        table.add_row(Row::from(vec![
            Cell::new(proto),
            Cell::new(port).fg(Color::Green),
            Cell::new(name).fg(Color::Yellow),
            Cell::new(pid).fg(Color::Magenta),
            user_cell,
            Cell::new(cwd),
            Cell::new(addr),
        ]));
    }

    println!("{table}");
}

pub fn handle_config(action: Option<ConfigAction>, json: bool) {
    let mut config = portly_core::load_config();
    let config_path = portly_core::get_config_path();

    match action.unwrap_or(ConfigAction::Show) {
        ConfigAction::Show => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&config).unwrap_or_default()
                );
            } else {
                println!("{}: {}", "Config Path".cyan().bold(), config_path.display());
                println!(
                    "{}: {}",
                    "Preferred Editor".bold(),
                    config
                        .preferred_editor
                        .as_deref()
                        .unwrap_or("auto-detect (cursor, code, zed)")
                );
                println!(
                    "{}: {}",
                    "Preferred Terminal".bold(),
                    config
                        .preferred_terminal
                        .as_deref()
                        .unwrap_or("auto-detect (ghostty, kitty, alacritty)")
                );
                println!(
                    "{}: {}s",
                    "Refresh Interval".bold(),
                    config.auto_refresh_secs
                );
                println!("{}: {}", "Use HTTPS Default".bold(), config.use_https);
                println!(
                    "{}: {:?}",
                    "Pinned Ports".bold(),
                    if config.pinned_ports.is_empty() {
                        "none".to_string()
                    } else {
                        format!("{:?}", config.pinned_ports)
                    }
                );
            }
        }
        ConfigAction::Path => {
            println!("{}", config_path.display());
        }
        ConfigAction::SetEditor { editor } => {
            config.preferred_editor = Some(editor.clone());
            if let Err(e) = portly_core::save_config(&config) {
                eprintln!("{}: {}", "Failed to save config".red().bold(), e);
                std::process::exit(1);
            }
            println!("Preferred editor updated to: {}", editor.green().bold());
        }
        ConfigAction::SetTerminal { terminal } => {
            config.preferred_terminal = Some(terminal.clone());
            if let Err(e) = portly_core::save_config(&config) {
                eprintln!("{}: {}", "Failed to save config".red().bold(), e);
                std::process::exit(1);
            }
            println!("Preferred terminal updated to: {}", terminal.green().bold());
        }
        ConfigAction::SetRefresh { interval } => {
            config.auto_refresh_secs = interval;
            if let Err(e) = portly_core::save_config(&config) {
                eprintln!("{}: {}", "Failed to save config".red().bold(), e);
                std::process::exit(1);
            }
            println!(
                "Refresh interval updated to: {}s",
                interval.to_string().green().bold()
            );
        }
        ConfigAction::SetHttps { enabled } => {
            config.use_https = enabled;
            if let Err(e) = portly_core::save_config(&config) {
                eprintln!("{}: {}", "Failed to save config".red().bold(), e);
                std::process::exit(1);
            }
            println!(
                "HTTPS default set to: {}",
                enabled.to_string().green().bold()
            );
        }
        ConfigAction::Pin { port } => {
            if !config.pinned_ports.contains(&port) {
                config.pinned_ports.push(port);
                if let Err(e) = portly_core::save_config(&config) {
                    eprintln!("{}: {}", "Failed to save config".red().bold(), e);
                    std::process::exit(1);
                }
            }
            println!("Pinned port {}", port.to_string().green().bold());
        }
        ConfigAction::Unpin { port } => {
            config.pinned_ports.retain(|&p| p != port);
            if let Err(e) = portly_core::save_config(&config) {
                eprintln!("{}: {}", "Failed to save config".red().bold(), e);
                std::process::exit(1);
            }
            println!("Unpinned port {}", port.to_string().yellow().bold());
        }
    }
}
