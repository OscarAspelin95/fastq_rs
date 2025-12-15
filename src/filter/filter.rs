use crate::common::{AppError, general_bufwriter, needletail_fastq_reader};
use crate::common::{mean_error_and_phred, nucleotide_counts};
use anyhow::Result;
use std::path::PathBuf;

pub fn fastq_filter(
    fastq: Option<PathBuf>,
    min_len: usize,
    max_len: usize,
    min_error: f64,
    max_error: f64,
    min_softmasked: usize,
    max_softmasked: usize,
    min_ambiguous: usize,
    max_ambiguous: usize,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let mut reader = needletail_fastq_reader(fastq)?;
    let mut writer = general_bufwriter(outfile)?;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        let record_seq = record.seq();
        let record_qual = record.qual().expect("No quality in record"); // Make this better.

        // Early return for too short/long reads.
        let record_len = record_seq.len();
        if record_len < min_len || record_len > max_len {
            continue;
        }

        // Early return for too low/high error rate.
        let (mean_error, _) = mean_error_and_phred(record_qual);
        if mean_error < min_error || mean_error > max_error {
            continue;
        }

        // Early return for too few/many softmasked or ambiguous nucleotides.
        let (_, num_softmasked, num_ambiguous) = nucleotide_counts(&record_seq);
        if num_softmasked < min_softmasked || num_softmasked > max_softmasked {
            continue;
        }
        if num_ambiguous < min_ambiguous || num_ambiguous > max_ambiguous {
            continue;
        }

        record.write(&mut writer, None)?;
    }

    Ok(())
}
