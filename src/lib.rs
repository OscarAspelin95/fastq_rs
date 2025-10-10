//! `fastq_rs` — general purpose toolkit for processing and manipulating fastq files.
//!
//! This crate provides utilities for common operations such as filtering, searching, sorting, trimming, etc.
//!
//! # Example
//! ```
//! use fastq_rs::stats::fastq_stats;
//! 
//! let fastq = PathBuf::from("reads.fastq.gz");
//! let outfile = PathBuf::from("stats.json");
//! fastq_stats(Some(fastq), Some(outfile)).unwrap();
//! ```
//!
//! See the documentation for details about each command.
pub mod args;
pub mod common;
pub mod concat;
pub mod filter;
pub mod fq2fa;
pub mod fq2tab;
pub mod grep;
pub mod head;
pub mod mock;
pub mod sample;
pub mod sanitize;
pub mod sort;
pub mod stats;
pub mod trim;
