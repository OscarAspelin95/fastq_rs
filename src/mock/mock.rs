use crate::common::{AppError, general_bufwriter, utils::PHRED_OFFSET};
use anyhow::Result;
use rand::{prelude::*, random_range, rng};
use std::path::PathBuf;

const NTS: [u8; 4] = [b'A', b'C', b'G', b'T'];

fn mock_fix_qual(fix: &Option<String>, actual_phred: u8) -> Option<Vec<u8>> {
    fix.as_ref().map(|s| {
        let mut v = Vec::with_capacity(s.len());
        for _ in 0..s.len() {
            v.push(actual_phred);
        }
        v
    })
}

fn validate_input_arguments(
    num_reads: usize,
    min_len: usize,
    max_len: usize,
    phred: u8,
) -> Result<(), AppError> {
    if num_reads == 0 {
        return Err(AppError::InvalidArgumentError(
            "num_reads must be greater than 0".to_string(),
        ));
    }

    if min_len == 0 {
        return Err(AppError::InvalidArgumentError(
            "min_len must be greater than 0".to_string(),
        ));
    }

    if max_len == 0 {
        return Err(AppError::InvalidArgumentError(
            "max_len must be greater than 0".to_string(),
        ));
    }

    if phred == 0 {
        return Err(AppError::InvalidArgumentError(
            "phred must be greater than 0".to_string(),
        ));
    }

    if max_len < min_len {
        return Err(AppError::InvalidArgumentError(
            "max_len must be greater than min_len".to_string(),
        ));
    }

    Ok(())
}
pub fn fastq_mock(
    num_reads: usize,
    min_len: usize,
    max_len: usize,
    phred: u8,
    prefix_seq: Option<String>,
    suffix_seq: Option<String>,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let mut writer = general_bufwriter(outfile)?;
    let actual_phred = phred + PHRED_OFFSET as u8;

    validate_input_arguments(num_reads, min_len, max_len, phred)?;

    let prefix_qual = mock_fix_qual(&prefix_seq, actual_phred);
    let suffix_qual = mock_fix_qual(&suffix_seq, actual_phred);

    // Pre-allocate buffers for seq and qual
    let mut seq: Vec<u8> = Vec::with_capacity(max_len);
    let mut qual: Vec<u8> = Vec::with_capacity(max_len);

    let mut rng = rng();

    for i in 0..num_reads {
        seq.clear();
        qual.clear();

        let seq_len = random_range(min_len..max_len);

        seq.extend((0..seq_len).map(|_| NTS[rng.random_range(0..NTS.len())]));
        qual.resize(seq_len, actual_phred);

        assert_eq!(seq.len(), qual.len());

        // Read name.
        writer.write_all(b"@read_")?;
        writer.write_all((i + 1).to_string().as_bytes())?;
        writer.write_all(b"\n")?;

        // Sequence
        if let Some(s) = prefix_seq.as_ref() {
            writer.write_all(s.as_bytes())?
        }
        writer.write_all(&seq)?;
        if let Some(s) = suffix_seq.as_ref() {
            writer.write_all(s.as_bytes())?
        }
        writer.write_all(b"\n")?;
        writer.write_all(b"+\n")?;

        // Qual
        if let Some(q) = prefix_qual.as_ref() {
            writer.write_all(q)?
        }
        writer.write_all(&qual)?;
        if let Some(q) = suffix_qual.as_ref() {
            writer.write_all(q)?
        }
        writer.write_all(b"\n")?;
    }

    writer.flush()?;

    Ok(())
}
