use crate::common::AppError;
use crate::common::{bio_fastq_reader, general_bufwriter, reverse_complement};
use memchr::memmem;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn trim_forward<'a>(seq: &'a [u8], forward_barcode: &'a [u8], mut margin: usize) -> Option<usize> {
    // Forward barcode:
    // * We take the first occurring hit (if exists).
    let mut forward_hit = memmem::find(seq, forward_barcode);

    let forward_hit_location = forward_hit
        .take_if(|h| h <= &mut margin)
        .map(|h| h + forward_barcode.len());

    return forward_hit_location;
}

fn trim_reverse<'a>(seq: &'a [u8], reverse_barcode: &'a [u8], margin: usize) -> Option<usize> {
    let mut reverse_hit = memmem::rfind(seq, reverse_barcode);
    let reverse_hit_location =
        reverse_hit.take_if(|h| h >= &mut (seq.len() - margin - reverse_barcode.len()));

    return reverse_hit_location;
}

pub fn fastq_trim(
    fastq: Option<PathBuf>,
    min_len: usize,
    trim_start: usize,
    trim_end: usize,
    barcodes_start: Option<Vec<String>>,
    barcodes_end: Option<Vec<String>>,
    barcode_mismatches: usize,
    barcode_margin: usize,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let reader = bio_fastq_reader(fastq).map_err(|_| AppError::FastqError)?;
    let writer = Arc::new(Mutex::new(
        general_bufwriter(outfile).map_err(|_| AppError::FastqError)?,
    ));

    let barcodes_start: Vec<String> = barcodes_start.unwrap_or(vec![]);

    // For reverse barcodes, we need to first reverse complement.
    let barcodes_end: Vec<String> = barcodes_end
        .as_ref()
        .map(|vec| {
            vec.iter()
                .map(|s| String::from_utf8(reverse_complement(s.as_bytes())).unwrap())
                .collect()
        })
        .unwrap_or(vec![]);

    reader.records().par_bridge().for_each(|record| {
        let record = match record {
            Ok(record) => record,
            Err(_) => return,
        };

        let mut seq = record.seq();
        let mut qual = record.qual();

        for barcode_start in &barcodes_start {
            let forward_hit = trim_forward(seq, barcode_start.as_bytes(), barcode_margin);

            match forward_hit {
                None => continue,
                Some(forward_hit_location) => {
                    seq = &seq[forward_hit_location..];
                    qual = &qual[forward_hit_location..];
                    break;
                }
            }
        }

        for barcode_end in &barcodes_end {
            let reverse_hit_location = trim_reverse(seq, barcode_end.as_bytes(), barcode_margin);

            match reverse_hit_location {
                None => continue,
                Some(reverse_hit_location) => {
                    seq = &seq[..reverse_hit_location];
                    qual = &qual[..reverse_hit_location];
                    break;
                }
            }
        }

        assert!(seq.len() == qual.len());

        // We want to trim the entire read.
        if trim_start >= seq.len() || trim_end >= seq.len() {
            return;
        }

        // We want to trim the entire read.
        if trim_start >= seq.len() - trim_end {
            return;
        }

        seq = &seq[trim_start..seq.len() - trim_end];
        qual = &qual[trim_start..qual.len() - trim_end];

        if seq.len() >= min_len {
            let mut w = writer.lock().unwrap();
            w.write(b"@").unwrap();
            w.write(record.id().as_bytes()).unwrap();
            w.write(b"\n").unwrap();
            w.write(seq).unwrap();
            w.write(b"\n").unwrap();
            w.write(b"+\n").unwrap();
            w.write(qual).unwrap();
            w.write(b"\n").unwrap();
        }
    });

    Ok(())
}
