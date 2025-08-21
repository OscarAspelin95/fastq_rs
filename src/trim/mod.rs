pub mod trim;
pub use trim::fastq_trim;

#[cfg(feature = "plot")]
pub mod plot;

#[cfg(feature = "plot")]
pub use plot::{generate_plots, tsv_to_df};
