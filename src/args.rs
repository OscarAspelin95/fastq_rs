use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct App {
    #[clap(subcommand)]
    pub command: SubCommand,

    #[clap(flatten)]
    pub global_opts: GlobalOpts,
}

#[derive(Debug, Args)]
pub struct GlobalOpts {
    #[clap(
        short,
        long,
        global = true,
        required = false,
        default_value_t = 8,
        help = "Not applicable to all subcommands."
    )]
    pub threads: usize,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Stats {
        #[clap(short, long)]
        fastq: PathBuf,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
}
