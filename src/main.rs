mod args;
mod subcommands;

use structopt::StructOpt;

use args::{Args, Subcommand};

fn main() {
    let args = Args::from_args();
    match args.subcommand {
        Subcommand::FocusedProgramCwd {} => {
            subcommands::focused_program_cwd();
        }
        Subcommand::FocusedProgramKill {} => {
            subcommands::focused_program_kill();
        }

        Subcommand::FocusedProgramPid {} => {
            subcommands::focused_program_pid();
        }
    }
}
