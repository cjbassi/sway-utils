use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,

    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
}

#[derive(StructOpt, Debug)]
pub enum Subcommand {
    /// Prints the cwd of the program running in the focused window.
    #[structopt(name = "focused-program-cwd")]
    FocusedProgramCwd {},

    /// Kills the focused program.
    #[structopt(name = "focused-program-kill")]
    FocusedProgramKill {},

    /// Prints the PID of the focused program
    #[structopt(name = "focused-program-pid")]
    FocusedProgramPid {},
}
