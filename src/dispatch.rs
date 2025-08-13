use crate::args::{App, SubCommand};
use crate::sort::fastq_sort;
use crate::stats::fastq_stats;

pub fn dispatch(args: App) {
    match args.command {
        SubCommand::Stats { fastq, outfile } => fastq_stats(&fastq, outfile).unwrap(),
        SubCommand::Sort {
            fastq,
            by,
            reverse,
            window_size,
            kmer_size,
            max_read_error,
            max_minimizer_error,
            outfile,
        } => fastq_sort(
            &fastq,
            &by,
            reverse,
            window_size,
            kmer_size,
            max_read_error,
            max_minimizer_error,
            outfile,
        )
        .unwrap(),
    }
}
