use crate::common::general_bufwriter;
use crate::common::needletail_fastq_reader;
use anyhow::Result;

use std::path::PathBuf;

pub fn fastq_fq2fa(fastq: Option<PathBuf>, outfile: Option<PathBuf>) -> Result<()> {
    let mut reader = needletail_fastq_reader(fastq)?;
    let mut writer = general_bufwriter(outfile)?;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        // Read id.
        writer.write_all(b">")?;
        writer.write_all(&record.id())?;
        writer.write_all(b"\n")?;

        // Read sequence.
        writer.write_all(&record.seq())?;
        writer.write_all(b"\n")?;
    }

    writer.flush()?;

    Ok(())
}
