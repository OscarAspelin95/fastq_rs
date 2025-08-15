use plotlars::{BoxPlot, Histogram, Plot, Rgb, ScatterPlot};
use polars::prelude::*;
use std::path::PathBuf;

pub enum PlotType {
    ReadScatter,
    ReadBox,
    ReadHist,
}

pub fn tsv_to_df(read_tsv: &PathBuf) -> DataFrame {
    // Assumes column header is first
    let df = LazyCsvReader::new(PlPath::new(read_tsv.to_str().unwrap()))
        .with_separator(b'\t')
        .with_has_header(true)
        .with_truncate_ragged_lines(true)
        .finish()
        .unwrap();

    return df
        .with_column(
            when(col("read_phred").gt_eq(30))
                .then(lit("Very High"))
                .when(col("read_phred").gt_eq(20))
                .then(lit("High"))
                .when(col("read_phred").gt_eq(10))
                .then(lit("Medium"))
                .otherwise(lit("Low"))
                .alias("quality"),
        )
        .collect()
        .expect("Failed to read and generate DataFrame.");
}

fn plot_read_scatter(df: &DataFrame, outfile: &PathBuf) {
    let builder = ScatterPlot::builder()
        .data(df)
        .x("read_length")
        // NOTE - does not seem to work.
        .x_title("Read Length")
        .y("read_phred")
        // NOTE - does not seem to work.
        .y_title("Mean Read Phred")
        .legend_title("Quality")
        .group("quality")
        .plot_title("Read scatter plot")
        .size(3)
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
    builder.write_html(outfile.to_str().expect("Failed to covert PathBuf to &str"));
}

fn plot_read_box(df: &DataFrame, outfile: &PathBuf) {
    let builder = BoxPlot::builder()
        .data(df)
        .group("quality")
        // x-axis.
        .labels("quality")
        .x_title("Quality")
        // y-axis.
        .values("read_length")
        .y_title("Read length")
        // Misc.
        .legend_title("Quality")
        .plot_title("Read Length")
        // NOTE - we currently cannot map groups to colors so this is not expected to always work.
        // most probably, color order is the same as the lecographical order of the categorical values.
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
    builder.write_html(outfile.to_str().expect("Failed to covert PathBuf to &str"));
}

fn plot_read_hist(df: &DataFrame, outfile: &PathBuf) {
    let builder = Histogram::builder()
        .data(&df)
        .x("read_length")
        .x_title("Read Length")
        .group("quality")
        .legend_title("Quality")
        .opacity(0.5)
        .plot_title("Read Length Histogram")
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
    builder.write_html(outfile.to_str().expect("Failed to covert PathBuf to &str"));
}

pub fn plot(df: &DataFrame, plot_type: PlotType, outfile: &PathBuf) {
    match plot_type {
        PlotType::ReadScatter => plot_read_scatter(df, outfile),
        PlotType::ReadBox => plot_read_box(df, outfile),
        PlotType::ReadHist => plot_read_hist(df, outfile),
    }
}
