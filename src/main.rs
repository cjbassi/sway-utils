mod args;
mod subcommands;

use structopt::StructOpt;

use args::{Args, Subcommand};

fn main() {
    let args = Args::from_args();
}
