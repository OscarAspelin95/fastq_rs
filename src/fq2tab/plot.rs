use plotlars::{Plot, ScatterPlot};
use polars::prelude::*;
use std::path::PathBuf;

pub fn generate_plots(outfile: Option<PathBuf>) {
    let stats_tsv = match outfile {
        Some(stats_tsv) => stats_tsv,
        None => return,
    };

    // Assumes column header is first
    let df = LazyCsvReader::new(PlPath::new(stats_tsv.to_str().unwrap()))
        .with_separator(b'\t')
        .with_has_header(true)
        .with_truncate_ragged_lines(true)
        .finish()
        .unwrap();

    let builder = ScatterPlot::builder()
        .data(&df.collect().unwrap())
        .x("read_length")
        .y("read_phred")
        .plot_title("Read scatter plot")
        .build();

    builder.plot();

    // builder.write_html("/home/oscar/github/test.html");
}
