// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use portly_cli::{execute_cli_command, Cli};

mod commands;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // If subcommands or CLI flags (except 'gui') are provided, execute CLI headlessly
    if args.len() > 1 && args[1] != "gui" {
        let cli = Cli::parse();
        execute_cli_command(cli);
        return;
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::scan_ports,
            commands::kill_process,
            commands::execute_action
        ])
        .run(tauri::generate_context!())
        .expect("error while running portly desktop application");
}
