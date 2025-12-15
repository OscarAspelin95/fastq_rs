use crate::common::AppError;
use anyhow::Result;
use bio::io::fastq::Reader;
use flate2::read::MultiGzDecoder;
use needletail::{FastxReader, parse_fastx_file, parse_fastx_stdin};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

const PLAIN: [&str; 2] = [".fastq", ".fq"];
const GZIP: [&str; 2] = [".fastq.gz", ".fq.gz"];

enum FastqType {
    Gzip,
    Plain,
}

fn validate_fastq(fastq: &PathBuf) -> Result<(&PathBuf, FastqType), AppError> {
    if !fastq.exists() {
        return Err(AppError::FileDoesNotExistError(fastq.to_owned()));
    }

    let fastq_str = fastq
        .to_str()
        .ok_or(AppError::PathBufConversionError(fastq.to_owned()))?;

    if PLAIN.iter().any(|extension| fastq_str.ends_with(extension)) {
        return Ok((fastq, FastqType::Plain));
    }

    if GZIP.iter().any(|extension| fastq_str.ends_with(extension)) {
        return Ok((fastq, FastqType::Gzip));
    }

    Err(AppError::InvalidFileExtension(fastq.to_owned()))
}

/// Bio does not automatically detect gzip, so we need to handle this manually.
/// I'm not sure this is actually thread safe, potentially.
pub fn bio_fastq_reader(fastq: Option<PathBuf>) -> Result<Reader<BufReader<Box<dyn Read + Send>>>> {
    let reader = match fastq {
        // If file is provided, validate and create reader based on gzip or plain.
        Some(fastq) => {
            let (fastq_file, fastq_type) = validate_fastq(&fastq)?;

            let f = File::open(fastq_file)?;

            let reader: Box<dyn Read + Send> = match fastq_type {
                FastqType::Gzip => Box::new(MultiGzDecoder::new(f)),
                FastqType::Plain => Box::new(f),
            };

            reader
        }

        // If stdin, we assume plain fastq. No gzip stream allowed.
        None => {
            

            Box::new(std::io::stdin())
        }
    };

    Ok(Reader::new(reader))
}

pub fn needletail_fastq_reader(fastq: Option<PathBuf>) -> Result<Box<dyn FastxReader>> {
    let reader = match fastq {
        Some(fastq) => {
            let (fastq_file, _) = validate_fastq(&fastq)?;

            // With the "compression feature, needletail detects gzip automatically."
            parse_fastx_file(fastq_file)?
        }
        None => parse_fastx_stdin()?,
    };

    Ok(reader)
}
