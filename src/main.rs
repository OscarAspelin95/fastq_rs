use clap::Parser;
use simple_logger::SimpleLogger;

mod args;
mod common;
mod sort;

mod stats;
use crate::args::App;

mod dispatch;
use dispatch::dispatch;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args: App = App::parse();

    dispatch(args);
}
