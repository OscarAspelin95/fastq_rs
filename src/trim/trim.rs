use crate::common::AppError;
use crate::common::{general_bufwriter, needletail_fastq_reader};
use std::path::PathBuf;

/// TODO - switch needletail for bio parser to enable multi-threading.
#[allow(unused)]
pub fn fastq_trim(
    fastq: Option<PathBuf>,
    min_len: usize,
    trim_start: usize,
    trim_end: usize,
    barcodes_start: Option<Vec<String>>,
    barcodes_end: Option<Vec<String>>,
    barcode_mismatches: usize,
    barcode_margin: usize,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let mut reader = needletail_fastq_reader(fastq).map_err(|_| AppError::FastqError)?;
    let mut writer = general_bufwriter(outfile).map_err(|_| AppError::FastqError)?;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        todo!();
    }

    Ok(())
}
