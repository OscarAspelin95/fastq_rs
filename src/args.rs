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
#[command(version, about = "General purpose fastq toolkit.", long_about = None)]
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
        fastq: Option<PathBuf>,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Sanitize {
        #[clap(short, long)]
        fastq: Option<PathBuf>,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Head {
        #[clap(short, long)]
        fastq: Option<PathBuf>,

        #[clap(short, long, default_value_t = 5)]
        num_reads: usize,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Grep {
        #[clap(short, long)]
        fastq: Option<PathBuf>,

        #[clap(short, long)]
        pattern: String,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Concat {
        #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
        fastqs: Vec<PathBuf>,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Filter {
        #[clap(short, long)]
        fastq: Option<PathBuf>,

        #[clap(short, long, default_value_t = 0)]
        min_len: usize,

        #[clap(short, long, default_value_t = usize::MAX)]
        max_len: usize,

        #[clap(short, long, default_value_t = 0.0)]
        min_error: f64,

        #[clap(short, long, default_value_t = 1.0)]
        max_error: f64,

        #[clap(short, long, default_value_t = 0)]
        min_softmasked: usize,

        #[clap(short, long, default_value_t = usize::MAX)]
        max_softmasked: usize,

        #[clap(short, long, default_value_t = 0)]
        min_ambiguous: usize,

        #[clap(short, long, default_value_t = usize::MAX)]
        max_ambiguous: usize,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Sort {
        #[clap(short, long)]
        fastq: Option<PathBuf>,

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
        fastq: Option<PathBuf>,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Fq2Tab {
        #[clap(short, long)]
        fastq: Option<PathBuf>,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Sample {
        #[clap(short, long)]
        fastq: Option<PathBuf>,

        #[clap(
            short,
            long,
            default_value_t = 1.0,
            help = "Values <= 1.0 means by fraction. Otherwise, samples by number of reads."
        )]
        by: f32,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
    Trim {
        #[clap(short, long)]
        fastq: Option<PathBuf>,

        #[clap(long, default_value_t = 0)]
        min_len: usize,

        #[clap(long, default_value_t = 0)]
        trim_start: usize,

        #[clap(long, default_value_t = 0)]
        trim_end: usize,

        #[clap(long, required = false, value_delimiter = ' ')]
        barcode_forward: Option<Vec<String>>,

        #[clap(long, required = false, value_delimiter = ' ')]
        barcode_reverse: Option<Vec<String>>,

        #[clap(long, default_value_t = 2)]
        max_mismatches: u8,

        #[clap(long, default_value_t = 10)]
        barcode_margin: usize,

        #[clap(short, long)]
        outfile: Option<PathBuf>,

        #[clap(short, long, default_value = "barcodes.tsv")]
        barcodes_tsv: PathBuf,
    },
    Mock {
        #[clap(short, long, default_value_t = 10)]
        num_reads: usize,

        #[clap(long, default_value_t = 1)]
        min_len: usize,

        #[clap(long, default_value_t = 10)]
        max_len: usize,

        #[clap(long, default_value_t = 30)]
        phred: u8,

        #[clap(long)]
        prefix_seq: Option<String>,

        #[clap(long)]
        suffix_seq: Option<String>,

        #[clap(short, long)]
        outfile: Option<PathBuf>,
    },
}
