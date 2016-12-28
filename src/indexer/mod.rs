use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use self::file_index::FileIndex;

mod file_index;

const NGRAM_LENGTH: usize = 3;

pub fn parse_file(filename: &str) -> Result<(), Box<Error>> {
    info!("parsing file: {}", filename);

    let file_handle = try!(File::open(filename));
    let mut reader = BufReader::new(file_handle);

    let mut buffer: String = String::new();
    let mut byte_offset: u64 = 0;

    while {
        let bytes_read: usize = try!(reader.read_line(&mut buffer));
        if bytes_read > 0 {
            let hash = try!(parse_line(&buffer, byte_offset));
            byte_offset += bytes_read as u64;
        }
        bytes_read > 0
    } {}
    println!("total bytes from file '{}': {}", filename, byte_offset);
    return Ok(());
}

fn parse_line(line: &str, byte_offset: u64) -> Result<HashMap<u32, Vec<u64>>, Box<Error>> {
    debug!("parsing line at offset {}", byte_offset);

    let mut ngram_hash: HashMap<u32, Vec<u64>> = HashMap::new();
    let mut key_hash: HashMap<u32, bool> = HashMap::new();

    let line_len: usize = line.len();

    // only 24 bits of the u32 are used, since at max 8-bit characters are used at a time
    for start_pos in 0..(line_len - NGRAM_LENGTH + 1) {
        let bytes = str_to_uint(&line[start_pos..(start_pos + NGRAM_LENGTH)]);
        if !key_hash.contains_key(&bytes) {
            key_hash.insert(bytes, true);
            if !ngram_hash.contains_key(&bytes) {
                ngram_hash.insert(bytes, Vec::new());
            }

            if let Some(offset_vec) = ngram_hash.get_mut(&bytes) {
                offset_vec.push(byte_offset);
            } else {
                error!("hash should have contained vector, but does not");
            }
        }
    }
    return Ok(ngram_hash);
}

fn str_to_uint(str_slice: &str) -> u32 {
    let bytes = str_slice.as_bytes();
    let mut uint: u32 = 0;
    for i in 0..(bytes.len()) {
        uint <<= 8;
        uint += bytes[i] as u32;
    }
    return uint;
}

#[cfg(test)]
mod tests {
    #[test]
    fn str_to_uint_works_for_length_3() -> () {
        let bytes = String::from_utf8(vec![95, 32, 69]).unwrap();
        let expected = (95 << 16) + (32 << 8) + 69;
        assert_eq!(expected, super::str_to_uint(&bytes));
    }

    #[test]
    fn str_to_uint_works_for_length_4() -> () {
        let bytes = String::from_utf8(vec![95, 32, 69, 10]).unwrap();
        let expected = (95 << 24) + (32 << 16) + (69 << 8) + 10;
        assert_eq!(expected, super::str_to_uint(&bytes));
    }

    #[test]
    fn parse_line_gives_correct_hash() -> () {
        let line = "abc de";
        let byte_offset = 12;
        let result = super::parse_line(&line, byte_offset);
        assert!(result.is_ok());

        let hash = result.unwrap();
        println!("keys");
        for key in hash.keys() {
            println!("{}", key);
        }
        let trigrams = ["abc", "bc ", "c d", " de"];
        for trigram in trigrams.iter() {
            let bytes = super::str_to_uint(trigram);
            assert!(hash.contains_key(&bytes));
        }
    }
}
