#[macro_use]
extern crate log;
extern crate fern;
extern crate time;
extern crate glob;

mod indexer;
mod types;
mod util;

use std::env;

fn main() {
    config_logger();

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        error!("filename expected");
        return;
    }

    let file_result = indexer::parse_file(&args[1]);
    match file_result {
        Ok(_) => {},
        Err(err) => { error!("failed to parse file {}, {}", &args[1], err.to_string()); }
    }
}

fn config_logger() {
    // TODO print warnings and errors to stderr
    let stdout_logger = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            format!("[{}]{} {}", level, time::now().strftime("[%Y-%m-%d][%H:%M:%S]").unwrap(), msg)
        }),
        output: vec![fern::OutputConfig::stdout()],
        level: log::LogLevelFilter::Trace,
    };

    if let Err(err) = fern::init_global_logger(stdout_logger, log::LogLevelFilter::Trace) {
        panic!(err);
    }
}


