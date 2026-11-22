use clap::Parser;
use portly_cli::{execute_cli_command, maybe_launch_bundled_gui, Cli};

fn main() {
    if maybe_launch_bundled_gui() {
        return;
    }

    let cli = Cli::parse();
    execute_cli_command(cli);
}
