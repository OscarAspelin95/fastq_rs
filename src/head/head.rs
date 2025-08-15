use crate::common::AppError;
use crate::common::{general_bufwriter, needletail_fastq_reader};
use std::path::PathBuf;

pub fn fastq_head(
    fastq: &PathBuf,
    num_reads: usize,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let mut reader = needletail_fastq_reader(&fastq).map_err(|_| AppError::FastqError)?;

    let mut writer = general_bufwriter(outfile).map_err(|_| AppError::FastqError)?;

    let mut n: usize = 0;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => {
                n += 1;
                record
            }
            Err(_) => continue,
        };

        record
            .write(&mut writer, None)
            .map_err(|_| AppError::FastqError)?;

        if n >= num_reads {
            break;
        }
    }

    Ok(())
}
