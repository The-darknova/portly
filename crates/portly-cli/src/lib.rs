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
        /// Port number to open
        port: u16,

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
    /// Launch the desktop GUI
    Gui,
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

pub fn handle_open(port: u16, https: bool) {
    println!(
        "Opening port {} in default browser...",
        port.to_string().green()
    );
    if let Err(e) = portly_core::actions::open_in_browser(port, https) {
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
