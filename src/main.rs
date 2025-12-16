use clap::Parser;
use log::error;
use rayon::ThreadPoolBuilder;
use simple_logger::SimpleLogger;

use fastq_rs::args::App;

mod dispatch;
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
