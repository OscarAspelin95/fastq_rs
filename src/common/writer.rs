use anyhow::Result;
use bio::io::fastq::Writer;
use flate2::Compression;
use flate2::write::GzEncoder;
use serde::Serialize;
use serde_json;
use std::io::Write;
use std::path::PathBuf;
use std::{fs::File, io::BufWriter};

use crate::common::AppError;

pub fn write_json<T: Serialize>(outfile: Option<PathBuf>, s: T) -> Result<(), AppError> {
    let writer = general_bufwriter(outfile)?;
    serde_json::to_writer(writer, &s)
        .map_err(|err| AppError::SerializationError(err.to_string()))?;

    Ok(())
}

pub fn general_bufwriter(outfile: Option<PathBuf>) -> Result<Box<dyn Write + Send>, AppError> {
    match outfile {
        Some(outfile) => {
            let f = File::create(&outfile)?;

            let extension = outfile
                .extension()
                .expect("Outfile is missing valid extension.")
                .to_str()
                .expect("Outfile is missing valid extension.");

            let writer = match extension {
                "gz" => Box::new(BufWriter::new(GzEncoder::new(f, Compression::fast())))
                    as Box<dyn Write + Send>,
                _ => Box::new(BufWriter::new(f)) as Box<dyn Write + Send>,
            };

            Ok(writer)
        }
        None => {
            let writer = BufWriter::new(std::io::stdout());
            Ok(Box::new(writer))
        }
    }
}

/// Writer specifically for bio::io::fastq::Records. Will check outfile:
/// * If Some(outfile) -> will write gzip fastq.
/// * If None -> will write plain fastq to stdout.
pub fn bio_fastq_writer(outfile: Option<PathBuf>) -> Result<Writer<Box<dyn Write>>, AppError> {
    let writer: Box<dyn Write> = match outfile {
        Some(path) => {
            let f = File::create(path)?;
            Box::new(BufWriter::new(GzEncoder::new(f, Compression::fast())))
        }
        None => Box::new(BufWriter::new(std::io::stdout())),
    };

    Ok(Writer::new(writer))
}
