use crate::common::{general_bufwriter, needletail_fastq_reader};
use anyhow::Result;
use std::path::PathBuf;

pub fn fastq_head(
    fastq: Option<PathBuf>,
    num_reads: usize,
    outfile: Option<PathBuf>,
) -> Result<()> {
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

        record.write(&mut writer, None)?;

        if n >= num_reads {
            break;
        }
    }

    Ok(())
}
