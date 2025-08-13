use crate::args::{App, SubCommand};
use crate::stats::fastq_stats;

pub fn dispatch(args: App) {
    match args.command {
        SubCommand::Stats { fastq, outfile } => fastq_stats(&fastq, outfile).unwrap(),
    }
}
