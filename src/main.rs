mod args;
mod subcommands;

use structopt::StructOpt;

use args::{Args, Subcommand};

fn main() {
    let args = Args::from_args();
    match args.subcommand {
        Subcommand::FocusedWindowPwd { terminal } => {
            subcommands::focused_window_pwd(&terminal);
        }
    }
}
