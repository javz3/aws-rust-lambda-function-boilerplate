use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Opt {
    #[clap(short, long)]
    pub region: Option<String>,
    #[clap(short, long)]
    pub queue: Option<String>,
    #[clap(short, long)]
    pub verbose: bool,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Send,
    Receive,
}
