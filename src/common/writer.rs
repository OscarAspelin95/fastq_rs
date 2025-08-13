use crate::common::AppError;
use bio::io::fastq::Writer;
use serde::Serialize;
use serde_json;
use std::io::Write;
use std::path::PathBuf;
use std::{fs::File, io::BufWriter};

pub fn write_json<T: Serialize>(outfile: Option<PathBuf>, s: T) -> Result<(), AppError> {
    let writer = get_bufwriter(outfile).map_err(|_| AppError::FastqError)?;
    serde_json::to_writer(writer, &s).unwrap();

    Ok(())
}

pub fn get_bufwriter(outfile: Option<PathBuf>) -> Result<Box<dyn Write>, AppError> {
    match outfile {
        Some(outfile) => {
            let f = File::create(outfile).map_err(|_| AppError::FastqError)?;
            let writer = BufWriter::new(f);

            Ok(Box::new(writer))
        }
        None => {
            let writer = BufWriter::new(std::io::stdout());
            Ok(Box::new(writer))
        }
    }
}

/// Meant for writing bio::io::Fasta::Record.
pub fn bio_fastq_writer(outfile: Option<PathBuf>) -> Result<Writer<Box<dyn Write>>, AppError> {
    match outfile {
        Some(outfile) => {
            let f = File::create(outfile).map_err(|_| AppError::FastqError)?;
            let writer = Writer::new(Box::new(BufWriter::new(f)) as Box<dyn Write>);
            return Ok(writer);
        }
        None => {
            let writer = Writer::new(Box::new(BufWriter::new(std::io::stdout())) as Box<dyn Write>);
            return Ok(writer);
        }
    }
}
