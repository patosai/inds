#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

mod indexer;
mod types;

use std::env;

fn main() {
    config_logger();

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        error!("filename expected");
        return;
    }

    let file_result = indexer::parse_file(&args[1]);
    if file_result.is_err() {
        error!("failed to parse file {}", &args[1]);
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


