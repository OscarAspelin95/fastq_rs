use crate::common::{general_bufwriter, needletail_fastq_reader};
use anyhow::Result;
use std::path::PathBuf;

pub fn fastq_sanitize(fastq: Option<PathBuf>, outfile: Option<PathBuf>) -> Result<()> {
    let mut reader = needletail_fastq_reader(fastq)?;
    let mut writer = general_bufwriter(outfile)?;

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
