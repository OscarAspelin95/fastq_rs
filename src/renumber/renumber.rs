use crate::common::{AppError, general_bufwriter, needletail_fastq_reader};
use anyhow::Result;
use std::path::PathBuf;

pub fn fastq_renumber(fastq: Option<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let mut reader = needletail_fastq_reader(fastq)?;
    let mut writer = general_bufwriter(outfile)?;

    let mut n: usize = 0;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => {
                n += 1;
                record
            }
            Err(_) => continue,
        };

        let read_id = format!("@read_{n}");

        writer.write_all(read_id.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.write_all(&record.seq())?;
        writer.write_all(b"\n")?;
        writer.write_all(b"+\n")?;
        writer.write_all(record.qual().unwrap())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}
