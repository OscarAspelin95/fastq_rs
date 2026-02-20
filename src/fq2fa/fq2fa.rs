use crate::errors::AppError;
use anyhow::Result;
use bio_utils_rs::io::{get_bufwriter, needletail_reader};

use std::path::PathBuf;

pub fn fastq_fq2fa(fastq: Option<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let mut reader = needletail_reader(fastq)?;
    let mut writer = get_bufwriter(outfile)?;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        // Read id.
        writer.write_all(b">")?;
        writer.write_all(record.id())?;
        writer.write_all(b"\n")?;

        // Read sequence.
        writer.write_all(&record.seq())?;
        writer.write_all(b"\n")?;
    }

    writer.flush()?;

    Ok(())
}
