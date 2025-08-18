use crate::args::{App, SubCommand};
use crate::filter::fastq_filter;
use crate::fq2fa::fastq_fq2fa;
use crate::fq2tab::fastq_fq2tab;
use crate::head::fastq_head;
use crate::sample::fastq_sample;
use crate::sanitize::fastq_sanitize;
use crate::sort::fastq_sort;
use crate::stats::fastq_stats;
use crate::trim::fastq_trim;

pub fn dispatch(args: App) {
    match args.command {
        SubCommand::Stats { fastq, outfile } => fastq_stats(&fastq, outfile).unwrap(),
        SubCommand::Sanitize { fastq, outfile } => fastq_sanitize(&fastq, outfile).unwrap(),
        SubCommand::Head {
            fastq,
            num_reads,
            outfile,
        } => fastq_head(&fastq, num_reads, outfile).unwrap(),
        SubCommand::Filter {
            fastq,
            min_len,
            max_len,
            min_error,
            max_error,
            min_softmasked,
            max_softmasked,
            min_ambiguous,
            max_ambiguous,
            outfile,
        } => fastq_filter(
            &fastq,
            min_len,
            max_len,
            min_error,
            max_error,
            min_softmasked,
            max_softmasked,
            min_ambiguous,
            max_ambiguous,
            outfile,
        )
        .unwrap(),
        SubCommand::Trim {
            fastq,
            min_len,
            trim_start,
            trim_end,
            barcode_start,
            barcode_end,
            barcode_mismatches,
            barcode_margin,
            outfile,
        } => fastq_trim(
            &fastq,
            min_len,
            trim_start,
            trim_end,
            barcode_start,
            barcode_end,
            barcode_mismatches,
            barcode_margin,
            outfile,
        )
        .unwrap(),
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
        SubCommand::Sample { fastq, by, outfile } => fastq_sample(&fastq, by, outfile).unwrap(),
    }
}
