# fastq_rs
🚧 Work in progress fastq toolkit, aiming to an alternative to [seqkit](https://github.com/shenwei356/seqkit/).

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation
Clone the repository or download the source code. Enter the fastq_rs directory and run:<br>

`cargo build --release`

The generated binary is available in `target/release/fastq_rs`.

## Plots
To enable plotting, compile with the `plot` feature.<br>

`cargo build --release --features plot`<br>

Plots will be automatically generated to applicable subcommands.

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

Note - if no file is provided, `fastq_rs` will read from stdin (plain FASTQ).

Optional arguments:
<pre>
<b>-o/--outfile</b> [stats.json] - Output file.
</pre>

### fastq_rs `sanitize`
🟡 Attempt to sanitize malformatted reads.

`fastq_rs sanitize --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fastq_rs `head`
🟡 Output the first `n` reads.

`fastq_rs head --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>
<b>-n/--num-seqs</b> [5] - Number of reads to output.

<b>-o/--outfile</b> [stdout] - Output file.
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
🟡 Convert FASTQ to a .tsv file with information about each read. If compiled with the `plot` feature, will generate a read scatter and boxplot.

`fastq_rs fq2-tab --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>
<b>-o/--outfile</b> [stdout] - Output file.
</pre>


### fastq_rs `filter`
🟡 Filter reads.

`fastq_rs filter --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>
<b>--min-len</b> [0] - Minimum allowed read length.

<b>--max-len</b> [usize::MAX] - Maximum allowed read length.

<b>--min-err</b> [0.0] - Minimum allowed mean read error.

<b>--max-err</b> [1.0] - Maximum allowed mean read error.

<b>--min-softmasked</b> [0] - Minimum allowed num softmasked bases.

<b>--max-softmasked</b> [usize::MAX] - Maximum allowed num softmasked bases.

<b>--min-ambiguous</b> [0] - Minimum allowed num ambiguous bases.

<b>--max-ambiguous</b> [usize::MAX] - Maximum allowed num ambiguous bases.

<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fastq_rs `sample`
🟡 (down)sample reads by fraction or number of reads.

`fastq_rs sample --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>
<b>-b/--by</b> [1.0] - How to sample. Inputs <= 1.0 signifies sampling by fraction. Inputs > 1.0 signifies sampling by (whole) number of reads.

<b>-o/--outfile</b> [stdout] - Output file.
</pre>

### fastq_rs `trim`
🟡 Trim reads through fuzzy search with ambiguous nucleotide support.

`fastq_rs trim --fastq <reads.fastq.gz> <optional_args>`

Optional arguments:
<pre>

<b>--min-len</b> [0] - Minimum read length for trimmed read to be outputted.

<b>--trim-start</b> [0] - Force trim this number of bases at the start of all reads.

<b>--trim-end</b> [0] - Force trim this number of bases at the end of all reads.

<b>--barcode-forward</b> [none] - Barcode(s) to trim at the start of the read. Must be provided in 5' -> 3' direction.

<b>--barcode-reverse</b> [none] - Barcode(s) to trim at the end of the read. Must be provided in 5' -> 3' direction.

<b>--max-mismatches</b> [2] - Allow this many mismatches between the barcode and the read.

<b>--barcode-margin</b> [10] - Allow the barcode to be located at most this number of bases from the start/end of the read.

<b>-o/--outfile</b> [stdout] - Output file.
</pre>
