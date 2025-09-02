use clap::Parser;
use rayon::ThreadPoolBuilder;
use simple_logger::SimpleLogger;

mod args;
mod common;
mod concat;
mod filter;
mod fq2fa;
mod fq2tab;
mod grep;
mod head;
mod mock;
mod sample;
mod sanitize;
mod sort;
mod stats;
mod trim;

use crate::args::App;

mod dispatch;
use dispatch::dispatch;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args: App = App::parse();

    ThreadPoolBuilder::new()
        .num_threads(args.global_opts.threads)
        .build_global()
        .expect("Failed to configure global thread pool.");

    dispatch(args);
}
