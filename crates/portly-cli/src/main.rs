use clap::Parser;
use portly_cli::{execute_cli_command, Cli};

fn main() {
    let cli = Cli::parse();
    execute_cli_command(cli);
}
