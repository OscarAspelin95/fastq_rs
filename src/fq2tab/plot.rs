use plotlars::{Plot, Rgb, ScatterPlot};
use polars::prelude::*;
use std::path::PathBuf;

pub enum PlotType {
    ReadScatter,
}

fn plot_read_scatter(outfile: Option<PathBuf>) {
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
        .expect("Failed to read provided .tsv file.")
        .with_column(
            when(col("read_phred").gt_eq(30))
                .then(lit("Very High"))
                .when(col("read_phred").gt_eq(20))
                .then(lit("High"))
                .when(col("read_phred").gt_eq(10))
                .then(lit("Medium"))
                .otherwise(lit("Low"))
                .alias("quality"),
        );

    let builder = ScatterPlot::builder()
        .data(
            &df.collect()
                .expect("Failed to convert LazyFrame to DataFrame."),
        )
        .x("read_length")
        // NOTE - does not seem to work.
        .x_title("Read Length")
        .y("read_phred")
        // NOTE - does not seem to work.
        .y_title("Mean Read Phred")
        .legend_title("Quality")
        .group("quality")
        .plot_title("Read scatter plot")
        // NOTE - we currently cannot map groups to colors
        // so this is not expected to always work.
        .colors(vec![
            // High
            Rgb(80, 200, 120), // Emerald Green.
            // Low
            Rgb(128, 0, 32), // Burgundy Red.
            // Medium
            Rgb(242, 140, 40), // Cadmium Orange.
            // Very High.
            Rgb(65, 105, 255), // Royal Blue.
        ])
        .build();

    builder.plot();

    // TODO - add builder.write_html().
}

pub fn plot(outfile: Option<PathBuf>, plot_type: PlotType) {
    match plot_type {
        PlotType::ReadScatter => plot_read_scatter(outfile),
    }
}
