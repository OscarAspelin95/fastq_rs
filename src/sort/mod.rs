pub mod sort;
pub use sort::fastq_sort;

pub mod sort_types;
pub use sort_types::{GcContent, Minimizer, ReadError, ReadLength, Score};
