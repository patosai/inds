#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;

mod indexer;

fn main() {
    env_logger::init().unwrap();

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
