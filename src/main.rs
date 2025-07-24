use clap::Parser;
use log::info;
use simple_logger::SimpleLogger;
use std::path::PathBuf;

mod fastq;
use fastq::parser::fastq_parser;

#[derive(Parser, Debug)]
#[command(version = "0.0.1", long_about = "Blazing fast fastq stats.")]
struct Args {
    #[arg(short, long)]
    fastq: PathBuf,

    #[arg(short, long)]
    outfile: PathBuf,
}

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("Running main.rs");
    let args: Args = Args::parse();

    // Extract stats for each read.
    info!("Parsing fastq...");
    fastq_parser(&args.fastq, &args.outfile);
}
