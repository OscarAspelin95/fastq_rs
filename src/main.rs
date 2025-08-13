use clap::Parser;
use log::info;
use simple_logger::SimpleLogger;

mod common;
mod stats;

mod args;
use crate::args::App;

mod dispatch;
use dispatch::dispatch;

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("Running main.rs");
    let args: App = App::parse();

    dispatch(args);
}
