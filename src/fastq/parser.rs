use crate::fastq::utils::{PHRED_TO_ERROR, error_to_phred};
use bio::io::fastq::Reader;
use flate2::read::MultiGzDecoder;
use log::{self, error};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::BufWriter;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex};
use std::{fs::File, io::BufReader, path::PathBuf};
use std::{u64, usize};

pub fn fastq_reader(
    fastq: &PathBuf,
) -> Result<Reader<BufReader<MultiGzDecoder<File>>>, std::io::Error> {
    let f = File::open(fastq)?;

    let reader = Reader::from_bufread(BufReader::new(MultiGzDecoder::new(f)));

    return Ok(reader);
}

#[derive(Serialize, Deserialize)]
struct FastqStats {
    num_reads: usize,
    num_bases: usize,
    mean_error: f64,
    mean_phred: u8,
    mean_len: usize,
    shortest: Vec<usize>,
    longest: Vec<usize>,
}

fn calculate_mean_mean_phred(mean_errors: &[f64]) -> (f64, u8) {
    let error_sum: f64 = mean_errors.iter().sum();
    let error_mean = error_sum / mean_errors.len() as f64;
    return (error_mean, error_to_phred(error_mean));
}

fn calculate_mean_len(mean_lenghts: &[usize]) -> usize {
    return mean_lenghts.iter().sum::<usize>() / mean_lenghts.len();
}

pub fn fastq_parser(fastq: &PathBuf, outfile: &PathBuf) {
    let reader = fastq_reader(&fastq);
    let records = reader.unwrap().records();

    // Initialize thread safe variables.
    let num_reads = AtomicUsize::new(0);
    let num_bases = AtomicUsize::new(0);
    let mean_errors: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));
    let read_lengths: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));

    records.par_bridge().for_each(|record| {
        // Skip invalid records and log error.
        let record = match record {
            Ok(record) => record,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        let record_len: usize = record.seq().len();

        let error_sum: f64 = record
            .qual()
            .iter()
            .map(|phred| {
                return PHRED_TO_ERROR[*phred as usize];
            })
            .sum();
        let mean_error = error_sum / record_len as f64;

        num_reads.fetch_add(1, Relaxed);
        num_bases.fetch_add(record_len, Relaxed);

        // Aggregate mean error and read length per read.
        mean_errors.lock().unwrap().push(mean_error);
        read_lengths.lock().unwrap().push(record_len);
    });

    // Mean phred.
    let mean_errors = Arc::try_unwrap(mean_errors).unwrap().into_inner().unwrap();
    let (mean_mean_error, mean_mean_phred) = calculate_mean_mean_phred(mean_errors.as_slice());

    // Mean read length.
    let mut read_lengths = Arc::try_unwrap(read_lengths).unwrap().into_inner().unwrap();
    read_lengths.par_sort();
    let mean_len = calculate_mean_len(read_lengths.as_slice());

    let f = FastqStats {
        num_reads: num_reads.into_inner(),
        num_bases: num_bases.into_inner(),
        mean_error: mean_mean_error,
        mean_phred: mean_mean_phred,
        mean_len: mean_len,
        shortest: read_lengths.first_chunk::<5>().unwrap().to_vec(),
        longest: read_lengths.last_chunk::<5>().unwrap().to_vec(),
    };

    // Write json to output file.
    let outfile = File::create(outfile).unwrap();
    let writer = BufWriter::new(outfile);
    serde_json::to_writer(writer, &f).unwrap();
}
