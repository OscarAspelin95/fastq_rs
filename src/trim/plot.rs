use plotlars::{BarPlot, ColorBar, Plot, Rgb};
use polars::prelude::*;
use std::path::PathBuf;

pub enum PlotType {
    TrimBar,
}

pub fn tsv_to_df(barcode_tsv: &PathBuf) -> DataFrame {
    // Assumes column header is first
    let df = LazyCsvReader::new(PlPath::new(barcode_tsv.to_str().unwrap()))
        .with_separator(b'\t')
        .with_has_header(true)
        .with_truncate_ragged_lines(true)
        .finish()
        .unwrap();

    return df
        .collect()
        .expect("Failed to read and generate DataFrame.");
}

fn plot_trim_bar(df: &DataFrame, outfile: &PathBuf) {
    let counts = df
        .clone()
        .lazy()
        .group_by([col("trimmed")])
        .agg([col("trimmed").count().alias("counts")])
        .collect()
        .unwrap();

    let builder = BarPlot::builder()
        .data(&counts)
        .group("trimmed")
        // x-axis.
        .labels("trimmed")
        .x_title("Trimmed")
        // y-axis.
        .values("counts")
        .y_title("Count")
        // Misc.
        .legend_title("Trimmed")
        .plot_title("Summary")
        .colors(vec![
            // ?
            Rgb(80, 200, 120), // Emerald Green.
            // ?
            Rgb(128, 0, 32), // Burgundy Red.
        ])
        .build();

    builder.plot();
    builder.write_html(outfile.to_str().expect("Failed to covert PathBuf to &str"));
}

fn plot(df: &DataFrame, plot_type: PlotType, outfile: &PathBuf) {
    match plot_type {
        PlotType::TrimBar => plot_trim_bar(df, outfile),
    }
}

pub fn generate_plots(barcodes_tsv: &PathBuf) {
    use crate::common::replace_extension;
    use crate::trim::plot::{PlotType, plot, tsv_to_df};

    let df = tsv_to_df(&barcodes_tsv);

    plot(
        &df,
        PlotType::TrimBar,
        &replace_extension(&barcodes_tsv, Some("bar"), "html"),
    );
}
