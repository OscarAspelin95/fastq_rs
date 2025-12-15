use crate::common::general_bufwriter;
use crate::common::mean_error_and_phred;
use crate::common::needletail_fastq_reader;
use anyhow::Result;
use std::path::PathBuf;

pub fn fastq_fq2tab(fastq: Option<PathBuf>, outfile: Option<PathBuf>) -> Result<()> {
    let mut reader = needletail_fastq_reader(fastq)?;
    let mut writer = general_bufwriter(outfile.clone())?;

    writer.write_all(b"read_id\tread_length\tread_error\tread_phred\n")?;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        let record_qual = match record.qual() {
            Some(record_qual) => record_qual,
            None => continue,
        };

        let record_seq = record.seq();

        let (mean_read_error, mean_read_phred) = mean_error_and_phred(record_qual);

        // Read id.
        writer.write_all(record.id())?;
        writer.write_all(b"\t")?;

        // Read length.
        writer.write_all(record_seq.len().to_string().as_bytes())?;
        writer.write_all(b"\t")?;

        // Read error
        writer.write_all(mean_read_error.to_string().as_bytes())?;
        writer.write_all(b"\t")?;

        // Read phred
        writer.write_all(mean_read_phred.to_string().as_bytes())?;
        writer.write_all(b"\t")?;
        writer.write_all(b"\n")?;
    }

    // Always remember to flush.
    writer.flush()?;

    Ok(())
}
