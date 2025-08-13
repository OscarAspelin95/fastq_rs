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
- [ ] Automatically extract sample name.
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
🟢 Sort reads based on provided metric.

`fastq_rs sort --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>

<b>-b/--by</b> [length] - {length, mean_error, minimizers}

<b>-r/--reverse</b> [false] - Sort in descending order.

<b>-w/--window-size</b> - Find a minimizer from this many consecutive kmers.

<b>-k/--kmer-size</b> - Kmer size to use.

<b>--max-read-error</b> - Reads with a mean error larger than this will be assigned zero valid minimizers.

<b>--max-minimizer-error</b> [0.05] - If sorting by minimizers, keep only minimizers that have at most this probability of being incorrect.

<b>-o/--outfile</b> [sorted.fastq.gz] - Output file.
</pre>

### fastq_rs `fq2fa`
🔴 Convert FASTQ to FASTA.

`fastq_rs fq2fa --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [reads.fasta] - Output file.
</pre>

### fastq_rs `fq2tab`
🔴 Convert FASTQ to a .tsv file with information about each read.

`fastq_rs fq2tab --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [reads.tsv] - Output file.
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

<b>-o/--outfile</b> [filtered.fastq.gz] - Output file.
</pre>
