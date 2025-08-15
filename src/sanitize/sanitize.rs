use crate::common::AppError;
use crate::common::{general_bufwriter, needletail_fastq_reader};
use std::path::PathBuf;

pub fn fastq_sanitize(fastq: &PathBuf, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let mut reader = needletail_fastq_reader(&fastq).map_err(|_| AppError::FastqError)?;

    let mut writer = general_bufwriter(outfile).map_err(|_| AppError::FastqError)?;

    while let Some(record) = reader.next() {
        match record {
            Ok(record) => {
                record
                    .write(&mut writer, None)
                    .map_err(|_| AppError::FastqError)?;
            }
            Err(_) => continue,
        };
    }

    Ok(())
}
