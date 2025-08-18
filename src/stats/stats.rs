use crate::common::utils::error_to_phred;
use crate::common::{bio_fastq_reader, mean_error_and_phred, mean_len, write_json};

use crate::common::AppError;

use log::error;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
struct FastqStats {
    num_reads: usize,
    num_bases: usize,
    mean_error: f64,
    mean_phred: u8,
    mean_len: usize,
    shortest: Option<Vec<usize>>,
    longest: Option<Vec<usize>>,
}

pub fn fastq_stats(fastq: Option<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let reader = bio_fastq_reader(fastq).map_err(|_| AppError::FastqError)?;

    // Initialize thread safe variables.
    let num_reads = AtomicUsize::new(0);
    let num_bases = AtomicUsize::new(0);
    let mean_errors: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));
    let read_lengths: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));

    reader.records().par_bridge().for_each(|record| {
        let record = match record {
            Ok(record) => record,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        let record_len: usize = record.seq().len();

        // Read error.
        let (mean_error, _) = mean_error_and_phred(&record.qual());

        num_reads.fetch_add(1, Relaxed);
        num_bases.fetch_add(record_len, Relaxed);

        // Aggregate mean error and read length per read.
        mean_errors.lock().unwrap().push(mean_error);
        read_lengths.lock().unwrap().push(record_len);
    });

    // NOTE that for performance reasons, we calculate the mean of the mean read error rates.
    // To get the true mean error, we'd have to store every single nucleotide error rate, sum
    // them up and divide by the total number of bases (unfeasible for large files).
    let mean_errors = Arc::try_unwrap(mean_errors).unwrap().into_inner().unwrap();
    let mean_mean_error = mean_errors.iter().sum::<f64>() / mean_errors.len() as f64;
    let mean_mean_phred = error_to_phred(mean_mean_error);

    // Mean read length.
    let mut read_lengths = Arc::try_unwrap(read_lengths).unwrap().into_inner().unwrap();
    read_lengths.par_sort();

    let mean_len = mean_len(read_lengths.as_slice());

    let fastq_stats = FastqStats {
        num_reads: num_reads.into_inner(),
        num_bases: num_bases.into_inner(),
        mean_error: mean_mean_error,
        mean_phred: mean_mean_phred,
        mean_len: mean_len,
        shortest: read_lengths
            .first_chunk::<5>()
            .and_then(|c| Some(c.to_vec())),
        longest: read_lengths
            .last_chunk::<5>()
            .and_then(|c| Some(c.to_vec())),
    };

    // Write json to output file.
    write_json(outfile, &fastq_stats)?;

    Ok(())
}
