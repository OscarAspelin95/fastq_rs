use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
pub enum SortType {
    Length,
    Gc,
    MeanError,
    Minimizer,
}

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
        default_value_t = 0,
        help = "Not applicable to all subcommands. By default set to 0, meaning Rayon will choose automatically."
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
    Sort {
        #[clap(short, long)]
        fastq: PathBuf,

        #[clap(value_enum, short, long, default_value_t = SortType::Length)]
        by: SortType,

        #[clap(short, long, default_value_t = false)]
        reverse: bool,

        #[clap(short, long, default_value_t = 10)]
        window_size: usize,

        #[clap(short, long, default_value_t = 15)]
        kmer_size: usize,

        #[clap(long, default_value_t = 0.05)]
        max_read_error: f64,

        #[clap(long, default_value_t = 0.05)]
        max_minimizer_error: f64,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Fq2Fa {
        #[clap(short, long)]
        fastq: PathBuf,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
}
