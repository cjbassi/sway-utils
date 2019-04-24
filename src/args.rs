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
    /// Prints the cwd of application running in the focused window.
    #[structopt(name = "focused-window-pwd")]
    FocusedWindowPwd {
        #[structopt(name = "terminal")]
        terminal: String,
    },
}
