use crate::args::SortType;
use crate::common::{bio_fastq_reader, bio_fastq_writer};
use crate::sort::{GcContent, Minimizer, ReadError, ReadLength, Score};

use anyhow::Result;
use bio::io::fastq::Record;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::path::PathBuf;

#[inline]
fn check_reverse(a: f64, b: f64, reverse: bool) -> Ordering {
    let ordering = a.partial_cmp(&b).expect("");

    match reverse {
        false => ordering,
        true => ordering.reverse(),
    }
}

pub fn fastq_sort(
    fastq: Option<PathBuf>,
    by: &SortType,
    reverse: bool,
    // Minimizer specific arguments.
    window_size: usize,
    kmer_size: usize,
    max_read_error: f64,
    max_minimizer_error: f64,
    //
    outfile: Option<PathBuf>,
) -> Result<()> {
    let reader = bio_fastq_reader(fastq)?;

    // Window size cannot be even, because Minimizer builder
    // will complain in this case (due to lexicographic tie breaking).
    let window_size = match window_size % 2 {
        0 => window_size + 1,
        _ => window_size,
    };

    let metric: Box<dyn Score> = match by {
        SortType::Length => Box::new(ReadLength {}),
        SortType::Gc => Box::new(GcContent {}),
        SortType::MeanError => Box::new(ReadError {}),
        SortType::Minimizer => Box::new(Minimizer {
            window_size: window_size,
            kmer_size: kmer_size,
            max_minimizer_error: max_minimizer_error,
            max_read_error: max_read_error,
        }),
    };

    let mut records_with_metrics: Vec<(f64, Record)> = reader
        .records()
        .par_bridge()
        .filter_map(|record| {
            let record = match record {
                Ok(record) => record,
                Err(_) => {
                    return None;
                }
            };

            let score = metric.score(&record.seq(), &record.qual());

            return Some((score, record));
        })
        .collect();

    records_with_metrics.par_sort_by(|a, b| check_reverse(a.0, b.0, reverse));

    let mut writer = bio_fastq_writer(outfile)?;

    for (_, record) in records_with_metrics {
        writer.write_record(&record)?;
    }

    writer.flush().expect("Failed to flush buffer.");

    Ok(())
}
