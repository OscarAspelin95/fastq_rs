use crate::common::AppError;
use crate::common::general_bufwriter;
use crate::common::mean_error_and_phred;
use crate::common::needletail_fastq_reader;
use std::path::PathBuf;

#[cfg(feature = "plot")]
use crate::fq2tab::plot::{PlotType, plot, tsv_to_df};

#[cfg(feature = "plot")]
fn generate_plots(outfile: Option<PathBuf>) {
    use crate::common::replace_extension;

    let read_tsv = match outfile {
        Some(read_tsv) => read_tsv,
        None => return,
    };

    let df = tsv_to_df(&read_tsv);

    plot(
        &df,
        PlotType::ReadScatter,
        &replace_extension(&read_tsv, Some("scatter"), "html"),
    );
    plot(
        &df,
        PlotType::ReadBox,
        &replace_extension(&read_tsv, Some("box"), "html"),
    );
}

pub fn fastq_fq2tab(fastq: &PathBuf, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let mut reader = needletail_fastq_reader(fastq).map_err(|_| AppError::FastqError)?;

    let mut writer = general_bufwriter(outfile.clone()).map_err(|_| AppError::FastqError)?;

    writer
        .write_all(b"read_id\tread_length\tread_error\tread_phred\n")
        .map_err(|_| AppError::FastqError)?;

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

        let (mean_read_error, mean_read_phred) = mean_error_and_phred(&record_qual);

        // Read id.
        writer
            .write_all(&record.id())
            .map_err(|_| AppError::FastqError)?;

        writer.write_all(b"\t").map_err(|_| AppError::FastqError)?;

        // Read length.
        writer
            .write_all(record_seq.len().to_string().as_bytes())
            .map_err(|_| AppError::FastqError)?;

        writer.write_all(b"\t").map_err(|_| AppError::FastqError)?;

        // Read error
        writer
            .write_all(mean_read_error.to_string().as_bytes())
            .map_err(|_| AppError::FastqError)?;

        writer.write_all(b"\t").map_err(|_| AppError::FastqError)?;

        // Read phred
        writer
            .write_all(mean_read_phred.to_string().as_bytes())
            .map_err(|_| AppError::FastqError)?;

        writer.write_all(b"\t").map_err(|_| AppError::FastqError)?;

        writer.write_all(b"\n").map_err(|_| AppError::FastqError)?;
    }

    writer.flush().map_err(|_| AppError::FastqError)?;

    #[cfg(feature = "plot")]
    generate_plots(outfile);

    Ok(())
}
