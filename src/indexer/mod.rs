mod binary;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use types::*;

const NGRAM_LENGTH: usize = 3;

pub fn parse_file(filename: &str) -> Result<(), Box<Error>> {
    let file_handle = try!(File::open(filename));
    let reader = BufReader::new(file_handle);
    info!("parsing file '{}'", filename);

    let mut byte_offset: ByteOffset = 0;
    let mut line_offsets: Vec<ByteOffset> = Vec::new();
    let mut line_count: LineNumber = 0;
    let mut ngram_hash: NgramHashMap = NgramHashMap::default();

    for result in reader.lines() {
        let line = try!(result);
        line_offsets.push(byte_offset);
        let bytes_parsed = parse_line(&line, line_count, &mut ngram_hash);

        byte_offset += bytes_parsed as ByteOffset;
        line_count += 1;

        if line_count % 1000 == 0 {
            debug!("{} bytes parsed", byte_offset);
        }
    }
    info!("{} bytes parsed", byte_offset);

    info!("building binary index");
    try!(binary::encoder::encode(&filename, &line_offsets, &ngram_hash));
    // TODO
    Ok(())
}

// splits line into multi-byte chunks
fn parse_line(line: &str, line_count: LineNumber, hash: &mut NgramHashMap) -> ByteOffset {
    let min_length = NGRAM_LENGTH;
    if line.len() <= min_length {
        warn!("not parsing line of length {}; too short", line.len());
        return 0;
    }

    let line_bytes = line.as_bytes();
    for start_pos in 0..(line_bytes.len() - NGRAM_LENGTH + 1) {
        let bytes: BinaryNgram = byte_arr_to_uint(&line_bytes[start_pos..(start_pos + NGRAM_LENGTH)]);
        let vec = hash.entry(bytes).or_insert(Vec::new());
        // don't duplicate
        if let Some(last_line_count) = vec.last() {
            if line_count.eq(last_line_count) {
                continue;
            }
        }
        vec.push(line_count);
    }

    line.len() as ByteOffset
}

fn byte_arr_to_uint(byte_arr: &[u8]) -> BinaryNgram {
    let mut uint: BinaryNgram = 0;
    for i in 0..(byte_arr.len()) {
        uint <<= 8;
        uint += byte_arr[i] as BinaryNgram;
    }

    uint
}

#[cfg(test)]
mod tests {
    use types::*;

    #[test]
    fn byte_arr_to_uint_works_for_length_3() {
        let bytes = vec![95, 32, 69];
        let expected = (95 << 16) + (32 << 8) + 69;
        assert_eq!(expected, super::byte_arr_to_uint(&bytes));
    }

    #[test]
    fn byte_arr_to_uint_works_for_length_4() {
        let bytes = vec![95, 32, 69, 10];
        let expected = (95 << 24) + (32 << 16) + (69 << 8) + 10;
        assert_eq!(expected, super::byte_arr_to_uint(&bytes));
    }

    #[test]
    fn parse_line_gives_nonduplicated_hash() {
        let line = "abc de";
        let byte_offset = 12;
        let mut hash: NgramHashMap = NgramHashMap::default();
        let result = super::parse_line(&line, byte_offset, &mut hash);
        assert_eq!(line.len() as u64, result);

        let trigrams = ["abc", "bc ", "c d", " de"];
        for trigram in trigrams.iter() {
            let bytes = super::byte_arr_to_uint(trigram.as_bytes());
            assert!(hash.contains_key(&bytes));
            assert_eq!(1, hash[&bytes].len());
            assert!(hash[&bytes].contains(&12u16));
        }
    }
}
