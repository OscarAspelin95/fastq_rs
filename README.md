# fastq_rs
🚧 Work in progress fastq toolkit, aiming to an alternative to [seqkit](https://github.com/shenwei356/seqkit/).

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation
Clone the repository or download the source code. Enter the fastq_rs directory and run:<br>
`cargo build --release`

The generated binary is available in `target/release/fastq_rs`.

## Usage
Run with:<br>
`fastq_rs <subcommand> <args>`<br>

## ToDo
- [ ] Improve error handling and add more error types.

## Subcommands
🔴 Not implemented yet (but planning to).<br>
🟡 Implemented but not tested/fully featured.<br>
🟢 Beta-mode available!

### fastq_rs `stats`
🟢 Calculate basic stats.

`fastq_rs stats --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [stats.json] - Output file.
</pre>

### fastq_rs `sort`
🟡 Sort reads based on provided metric.

`fastq_rs sort --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>

<b>-b/--by</b> [length] - {length, gc_content, mean_error, minimizers}

<b>-r/--reverse</b> [false] - Sort in descending order.

<b>-w/--window-size</b> [10] - Minimizer window size (number of consecutive kmers).

<b>-k/--kmer-size</b> [15] - Minimizer kmer size.

<b>--max-read-error</b> [0.05] - Minimizer max allowed read error. Reads with higher error rates will generate zero valid minimizers.

<b>--max-minimizer-error</b> [0.05] - Minimizer error probability cutoff.

<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fastq_rs `fq2-fa`
🟡 Convert FASTQ to FASTA.

`fastq_rs fq2-fa --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fastq_rs `fq2-tab`
🔴 Convert FASTQ to a .tsv file with information about each read.

`fastq_rs fq2-tab --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fastq_rs `filter`
🔴 Filter reads.

`fastq_rs filter --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>
<b>--min-len</b> [0] - Minimum allowed read length.

<b>--max-len</b> [usize::MAX] - Maximum allowed read length.

<b>--min-err</b> [0.0] - Minimum allowed mean read error.

<b>--max-err</b> [1.0] - Maximum allowed mean read error.

<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fastq_rs `trim`
🔴 Trim reads.

`fastq_rs trim --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>
<b>-w/--window-size</b> [10] - Sliding window to check and trim.

<b>--max-error</b> [0.05] - Maximum allowed error in window. Higher error windows will be trimmed.

<b>--barcodes-forward</b> [none] - 5'-3' barcodes to trim at the start of reads.

<b>--barcodes-reverse</b> [none] - 5'-3' barcodes to trim at the end of reads.

<b>--barcode-mismatches</b> [2] - Allow this number of mismatches between barcode and read.

<b>--barcode-margin</b> [10] - Allow barcode to match within this distance from the start/end of read.

<b>-o/--outfile</b> [stdout] - Output file.
</pre>
