use crate::args::{App, SubCommand};
use crate::fq2fa::fastq_fq2fa;
use crate::fq2tab::fastq_fq2tab;
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
        SubCommand::Fq2Fa { fastq, outfile } => fastq_fq2fa(&fastq, outfile).unwrap(),
        SubCommand::Fq2Tab { fastq, outfile } => fastq_fq2tab(&fastq, outfile).unwrap(),
    }
}
