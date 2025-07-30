# fastq_rs
Blazingly fast fastq parser suitable for Nanopore and PacBio data.

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation
Clone the repository or download the source code. Enter the fastq_rs directory and run:<br>
`cargo build --release`

The generated binary is available in `target/release/fastq_rs`.

## Usage
Run with:<br>
`fastq_rs --fastq <fastq> --outfile <out.json>`
