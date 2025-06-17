use crate::fastq::utils::{PHRED_TO_ERROR, error_to_phred};
use bio::io::fastq::Reader;
use flate2::read::MultiGzDecoder;
use rayon::prelude::*;
use std::io::{BufWriter, Write, stdout};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::{fs::File, io::BufReader, path::PathBuf};

pub fn fastq_reader(
    fastq: &PathBuf,
) -> Result<Reader<BufReader<MultiGzDecoder<File>>>, std::io::Error> {
    let f = File::open(fastq)?;

    let reader = Reader::from_bufread(BufReader::new(MultiGzDecoder::new(f)));

    return Ok(reader);
}

pub fn fastq_parser(fastq: &PathBuf, min_len: usize, min_phred: u8) {
    let reader = fastq_reader(&fastq);
    let records = reader.unwrap().records();

    let (tx, rx) = std::sync::mpsc::channel();

    // Thread safe way to keep track of num reads and num bases.
    let num_reads = AtomicUsize::new(0);
    let num_bases = AtomicUsize::new(0);

    records.par_bridge().for_each(|record| {
        let record = record.unwrap();
        let record_len = record.seq().len();

        // Calculate the mean error rate for the read
        // NOTE - assumes phred 33 offset.
        let error_sum: f64 = record
            .qual()
            .iter()
            .map(|phred| {
                return PHRED_TO_ERROR[*phred as usize];
            })
            .sum();

        let mean_error = error_sum / record_len as f64;
        let mean_phred = error_to_phred(mean_error);

        if record_len >= min_len && mean_phred >= min_phred {
            let mut buffer = String::new();

            // Send valid fastq record to sender.
            buffer.push_str(&format!("@{}\n", record.id()));
            buffer.push_str(&String::from_utf8_lossy(record.seq()));
            buffer.push_str("\n+\n");
            buffer.push_str(&String::from_utf8_lossy(record.qual()));
            buffer.push_str("\n");

            tx.send(buffer).unwrap();

            num_reads.fetch_add(1, Relaxed);
            num_bases.fetch_add(record_len, Relaxed);
        }
    });

    drop(tx);

    let mut writer = BufWriter::new(stdout());
    for buffer in rx {
        writer.write_all(buffer.as_bytes()).unwrap();
    }
}
