use crate::errors::AppError;
use bio_utils_rs::io::{get_bufwriter, needletail_reader};
use std::path::PathBuf;

pub fn fastq_concat(fastqs: Vec<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let mut writer = get_bufwriter(outfile)?;

    for fastq in fastqs {
        let mut reader = needletail_reader(Some(fastq))?;

        while let Some(record) = reader.next() {
            let record = match record {
                Ok(record) => record,
                Err(_) => continue,
            };

            record.write(&mut writer, None)?;
        }
    }

    Ok(())
}
