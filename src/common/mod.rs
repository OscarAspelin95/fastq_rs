pub mod utils;
pub use utils::{PHRED_TO_ERROR, mean_error_and_phred, mean_len};

pub mod errors;
pub use errors::AppError;

pub mod writer;
pub use writer::{bio_fastq_writer, general_bufwriter, write_json};

pub mod reader;
pub use reader::bio_fastq_reader;
