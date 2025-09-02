use crate::common::{general_bufwriter, needletail_fastq_reader};
use anyhow::Result;
use std::path::PathBuf;

pub fn fastq_concat(fastqs: Vec<PathBuf>, outfile: Option<PathBuf>) -> Result<()> {
    let mut writer = general_bufwriter(outfile)?;

    for fastq in fastqs {
        let mut reader = needletail_fastq_reader(Some(fastq))?;

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
