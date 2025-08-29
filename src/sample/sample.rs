use crate::common::AppError;
use crate::common::{bio_fastq_reader, bio_fastq_writer};
use anyhow::Result;
use bio::io::fastq::Record;
use rand::{prelude::*, rng};
use std::path::PathBuf;

pub fn fastq_sample(fastq: Option<PathBuf>, by: f32, outfile: Option<PathBuf>) -> Result<()> {
    let reader = bio_fastq_reader(fastq)?;
    let mut writer = bio_fastq_writer(outfile)?;

    let records: Vec<Record> = reader
        .records()
        .filter_map(|record| match record {
            Ok(record) => Some(record),
            Err(_) => None,
        })
        .collect();

    // Check for valid sampling metric.
    if by <= 0.0 {
        return Err(AppError::InvalidSamplingError(by).into());
    }

    let sample_by = match by <= 1.0 {
        // Sample by fraction.
        true => {
            let num_reads = (by * records.len() as f32) as usize;
            std::cmp::max(1, num_reads)
        }
        // Sample by number.
        false => std::cmp::min(records.len(), by as usize),
    };

    let mut rng = rng();
    let sample = records.choose_multiple(&mut rng, sample_by);

    for r in sample {
        writer.write_record(r)?;
    }

    Ok(())
}
