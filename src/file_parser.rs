use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

pub fn read_file(filename: &str) -> () {
    let file_handle = File::open(filename).unwrap();
    let reader = BufReader::new(file_handle);

    let mut num_lines = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        println!("{}", l);
        num_lines += 1;
    }
    println!("Total linecount: {}", num_lines);
}
