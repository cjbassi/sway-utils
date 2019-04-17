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
    /// todo
    #[structopt(name = "todo")]
    Todo {
        #[structopt(name = "days")]
        todo: Option<f64>,
    },
}
