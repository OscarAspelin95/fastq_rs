use crate::common::AppError;
use crate::common::general_bufwriter;
use crate::common::needletail_fastq_reader;

use std::path::PathBuf;

pub fn fastq_fq2fa(fastq: &PathBuf, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let mut reader = needletail_fastq_reader(fastq).map_err(|_| AppError::FastqError)?;

    let mut writer = general_bufwriter(outfile).map_err(|_| AppError::FastqError)?;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        // Read id.
        writer.write_all(b">").map_err(|_| AppError::FastqError)?;
        writer
            .write_all(&record.id())
            .map_err(|_| AppError::FastqError)?;

        writer.write_all(b"\n").map_err(|_| AppError::FastqError)?;

        // Read sequence.
        writer
            .write_all(&record.seq())
            .map_err(|_| AppError::FastqError)?;

        writer.write_all(b"\n").map_err(|_| AppError::FastqError)?;
    }

    writer.flush().map_err(|_| AppError::FastqError)?;

    Ok(())
}
