use clap::Parser;
use log::error;
use rayon::ThreadPoolBuilder;
use simple_logger::SimpleLogger;

mod args;
mod concat;
mod dispatch;
mod errors;
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

use args::App;
use dispatch::dispatch;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args: App = App::parse();

    ThreadPoolBuilder::new()
        .num_threads(args.global_opts.threads)
        .build_global()
        .expect("Failed to configure global thread pool.");

    let result = dispatch(args);

    match result {
        Ok(_) => {}
        Err(e) => error!("Error: {}", e),
    }
}
