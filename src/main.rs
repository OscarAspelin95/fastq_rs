use clap::Parser;
use log::info;
use simple_logger::SimpleLogger;
use std::path::PathBuf;

mod fastq;
use fastq::parser::fastq_parser;

#[derive(Parser, Debug)]
#[command(version = "0.0.1", long_about = "Module for ")]
struct Args {
    #[arg(long)]
    fastq: PathBuf,

    #[arg(long)]
    min_len: usize,

    #[arg(long)]
    min_phred: u8,
}

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("Running main.rs");

    let args: Args = Args::parse();

    // Extract stats for each read.
    info!("Parsing fastq...");
    fastq_parser(&args.fastq, args.min_len, args.min_phred);
}
