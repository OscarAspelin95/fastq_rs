use crate::common::{bio_fastq_reader, general_bufwriter, reverse_complement};
use anyhow::Result;
use bio::pattern_matching::myers::MyersBuilder;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[cfg(feature = "plot")]
use crate::trim::generate_plots;

// Allow for ambiguous nucleotide matches.
#[inline]
fn myers_builder(primer_seq: &[u8]) -> bio::pattern_matching::myers::Myers {
    return MyersBuilder::new()
        .ambig(b'N', b"ACGT")
        .ambig(b'R', b"AG")
        .ambig(b'Y', b"CT")
        .ambig(b'S', b"GC")
        .ambig(b'W', b"AT")
        .ambig(b'K', b"GT")
        .ambig(b'M', b"AC")
        .ambig(b'B', b"CGT")
        .ambig(b'D', b"AGT")
        .ambig(b'H', b"ACT")
        .ambig(b'V', b"ACG")
        .build_64(primer_seq);
}

fn find_fuzzy<'a>(seq: &[u8], barcode: &'a [u8], max_mismatches: u8) -> Option<usize> {
    let myers = myers_builder(barcode);

    let (start, num_mismatches) = myers.find_best_end(seq);

    if num_mismatches <= max_mismatches {
        return Some(start);
    }

    return None;
}

/// I'm not happy with the multi-thread implementation with
/// lots of Arcs and Mutexes floating around. It might be the
/// case that needletail single-thread is actually faster.
pub fn fastq_trim(
    fastq: Option<PathBuf>,
    min_len: usize,
    trim_start: usize,
    trim_end: usize,
    barcodes_forward: Option<Vec<String>>,
    barcodes_reverse: Option<Vec<String>>,
    max_mismatches: u8,
    barcode_margin: usize,
    outfile: Option<PathBuf>,
    barcodes_tsv: PathBuf,
) -> Result<()> {
    // Fastq reader/writer.
    let reader = bio_fastq_reader(fastq)?;
    let fastq_writer = Arc::new(Mutex::new(general_bufwriter(outfile)?));

    // Tsv writer (to file).
    let tsv_writer = Arc::new(Mutex::new(general_bufwriter(Some(barcodes_tsv.clone()))?));

    // If not supplied, empty vec means no iterating.
    let barcodes_start: Vec<String> = barcodes_forward.unwrap_or(vec![]);

    // For reverse barcodes, we need to first reverse complement.
    let barcodes_end: Vec<String> = barcodes_reverse
        .as_ref()
        .map(|vec| {
            vec.iter()
                .map(|s| String::from_utf8(reverse_complement(s.as_bytes())).unwrap())
                .collect()
        })
        .unwrap_or(vec![]);

    // Writer tsv header
    {
        let mut s = tsv_writer.lock().unwrap();
        s.write(
            b"read_name\tlength_before\tlength_after\ttrimmed\tbarcode_forward\tbarcode_reverse\n",
        )
        .unwrap();
    }

    reader.records().par_bridge().for_each(|record| {
        let record = match record {
            Ok(record) => record,
            Err(_) => return,
        };

        let mut seq = record.seq();
        let mut qual = record.qual();
        let mut trimmed: bool = false;
        let mut found_barcode_forward: Option<&[u8]> = None;
        let mut found_barcode_reverse: Option<&[u8]> = None;

        for barcode_forward in &barcodes_start {
            let barcode_len = barcode_forward.len();
            let total_margin = barcode_len + barcode_margin + 2;

            // Skip too short sequences.
            if seq.len() <= total_margin {
                continue;
            }

            // Only look in relevant part of seq.
            let forward_start = find_fuzzy(
                &seq[..total_margin],
                barcode_forward.as_bytes(),
                max_mismatches,
            );

            match forward_start {
                None => continue,
                Some(forward_start) => {
                    seq = &seq[forward_start + barcode_len..];
                    qual = &qual[forward_start + barcode_len..];
                    found_barcode_forward = Some(barcode_forward.as_bytes());
                    trimmed = true;

                    break;
                }
            }
        }

        for barcode_reverse in &barcodes_end {
            let barcode_len = barcode_reverse.len();
            let seq_len = seq.len();

            let total_margin: usize = barcode_len + barcode_margin + 2;

            // Skip too short sequences.
            if seq_len <= total_margin {
                continue;
            }

            // Only look in relevant part of seq.
            let reverse_start = find_fuzzy(
                &seq[seq_len - total_margin..],
                barcode_reverse.as_bytes(),
                max_mismatches,
            );

            match reverse_start {
                None => continue,
                Some(reverse_start) => {
                    seq = &seq[..seq_len - reverse_start];
                    qual = &qual[..seq_len - reverse_start];
                    found_barcode_reverse = Some(barcode_reverse.as_bytes());
                    trimmed = true;

                    break;
                }
            }
        }

        // We want to hard-trim the entire remaining seq.
        if trim_start >= seq.len() || trim_end >= seq.len() {
            return;
        }

        // We want to hard-trim the entire remaining seq.
        if trim_start >= seq.len() - trim_end {
            return;
        }

        seq = &seq[trim_start..seq.len() - trim_end];
        qual = &qual[trim_start..qual.len() - trim_end];

        if seq.len() >= min_len {
            let mut w = fastq_writer.lock().unwrap();
            w.write(b"@").unwrap();
            w.write(record.id().as_bytes()).unwrap();
            w.write(b"\n").unwrap();
            w.write(seq).unwrap();
            w.write(b"\n").unwrap();
            w.write(b"+\n").unwrap();
            w.write(qual).unwrap();
            w.write(b"\n").unwrap();
        }

        let mut s = tsv_writer.lock().unwrap();

        // Id.
        s.write(record.id().as_bytes()).unwrap();
        s.write(b"\t").unwrap();

        // Length before.
        s.write(record.seq().len().to_string().as_bytes()).unwrap();
        s.write(b"\t").unwrap();

        // Length after.
        s.write(seq.len().to_string().as_bytes()).unwrap();
        s.write(b"\t").unwrap();

        // Was trimmed?
        s.write(trimmed.to_string().as_bytes()).unwrap();
        s.write(b"\t").unwrap();

        // Forward barcode.
        let bf = found_barcode_forward.unwrap_or(b"N/A");
        s.write(bf).unwrap();
        s.write(b"\t").unwrap();

        // Reverse barcode.
        let br = found_barcode_reverse.unwrap_or(b"N/A");
        s.write(br).unwrap();
        s.write(b"\n").unwrap();
    });

    // ALWAYS remember to flush, otherwise you might spending hrs debugging...
    let mut tsv_writer = Arc::into_inner(tsv_writer).unwrap().into_inner().unwrap();
    tsv_writer.flush().unwrap();

    let mut fastq_writer = Arc::into_inner(fastq_writer).unwrap().into_inner().unwrap();
    fastq_writer.flush().unwrap();

    #[cfg(feature = "plot")]
    generate_plots(&barcodes_tsv);

    Ok(())
}
