use crate::errors::AppError;
use anyhow::Result;
use bio_utils_rs::io::{get_bufwriter, needletail_reader};
use std::path::PathBuf;

pub fn fastq_sanitize(fastq: Option<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let mut reader = needletail_reader(fastq)?;
    let mut writer = get_bufwriter(outfile)?;

    while let Some(record) = reader.next() {
        match record {
            Ok(record) => {
                record.write(&mut writer, None)?;
            }
            Err(_) => continue,
        };
    }

    Ok(())
}
