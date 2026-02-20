use crate::errors::AppError;
use anyhow::Result;
use bio_utils_rs::io::{get_bufwriter, needletail_reader};
use std::path::PathBuf;

pub fn fastq_head(
    fastq: Option<PathBuf>,
    num_reads: usize,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let mut reader = needletail_reader(fastq)?;
    let mut writer = get_bufwriter(outfile)?;

    let mut n: usize = 0;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => {
                n += 1;
                record
            }
            Err(_) => continue,
        };

        record.write(&mut writer, None)?;

        if n >= num_reads {
            break;
        }
    }

    Ok(())
}
