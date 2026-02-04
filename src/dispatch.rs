use anyhow::Result;
use fastq_rs::args::{App, SubCommand};
use fastq_rs::common::AppError;
use fastq_rs::concat::fastq_concat;
use fastq_rs::filter::fastq_filter;
use fastq_rs::fq2fa::fastq_fq2fa;
use fastq_rs::fq2tab::fastq_fq2tab;
use fastq_rs::grep::fastq_grep;
use fastq_rs::head::fastq_head;
use fastq_rs::mock::fastq_mock;
use fastq_rs::renumber::fastq_renumber;
use fastq_rs::sample::fastq_sample;
use fastq_rs::sanitize::fastq_sanitize;
use fastq_rs::sort::fastq_sort;
use fastq_rs::stats::fastq_stats;
use fastq_rs::trim::fastq_trim;

pub fn dispatch(args: App) -> Result<(), AppError> {
    match args.command {
        SubCommand::Stats { fastq, outfile } => {
            let _ = fastq_stats(fastq, outfile)?;
        }
        SubCommand::Sanitize { fastq, outfile } => fastq_sanitize(fastq, outfile)?,
        SubCommand::Head {
            fastq,
            num_reads,
            outfile,
        } => fastq_head(fastq, num_reads, outfile)?,
        SubCommand::Grep {
            fastq,
            pattern,
            outfile,
        } => fastq_grep(fastq, pattern, outfile)?,
        SubCommand::Concat { fastqs, outfile } => fastq_concat(fastqs, outfile)?,
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
        )?,
        SubCommand::Trim {
            fastq,
            min_len,
            trim_start,
            trim_end,
            barcode_forward,
            barcode_reverse,
            max_mismatches,
            barcode_margin,
            outfile,
            barcodes_tsv,
        } => fastq_trim(
            fastq,
            min_len,
            trim_start,
            trim_end,
            barcode_forward,
            barcode_reverse,
            max_mismatches,
            barcode_margin,
            outfile,
            barcodes_tsv,
        )?,
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
            fastq,
            &by,
            reverse,
            window_size,
            kmer_size,
            max_read_error,
            max_minimizer_error,
            outfile,
        )?,
        SubCommand::Fq2Fa { fastq, outfile } => fastq_fq2fa(fastq, outfile)?,
        SubCommand::Fq2Tab { fastq, outfile } => fastq_fq2tab(fastq, outfile)?,
        SubCommand::Sample { fastq, by, outfile } => fastq_sample(fastq, by, outfile)?,
        SubCommand::Mock {
            num_reads,
            min_len,
            max_len,
            phred,
            prefix_seq,
            suffix_seq,
            outfile,
        } => fastq_mock(
            num_reads, min_len, max_len, phred, prefix_seq, suffix_seq, outfile,
        )?,
        SubCommand::Renumber { fastq, outfile } => fastq_renumber(fastq, outfile)?,
    }

    Ok(())
}
