use crate::common::{AppError, general_bufwriter, needletail_fastq_reader};
use anyhow::Result;
use regex::Regex;
use std::path::PathBuf;

pub fn fastq_grep(
    fastq: Option<PathBuf>,
    pattern: String,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let mut reader = needletail_fastq_reader(fastq)?;
    let mut writer = general_bufwriter(outfile)?;

    let pattern =
        Regex::new(pattern.as_str()).map_err(|err| AppError::RegexParsingError(err.to_string()))?;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        let haystack = std::str::from_utf8(record.id())?;
        match pattern.captures(haystack) {
            Some(_) => record.write(&mut writer, None)?,
            None => continue,
        }
    }

    Ok(())
}
