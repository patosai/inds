use std::ffi::OsString;
use std::path::PathBuf;

use types::*;

fn add_special_extension(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(&filename);
    let mut extension = path
        .extension()
        .map_or(OsString::new(), |e| {
            let mut e = e.to_os_string();
            e.push(".");
            e
        });
    extension.push(&BINARY_FILE_SUFFIX);
    path.set_extension(&extension);
    path
}

pub mod encoder {
    use util;

    use std;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::PathBuf;

    use types::*;

    pub fn encode(filename: &str, line_offsets: &[ByteOffset], ngram_hash: &NgramHashMap) -> std::io::Result<()> {
        let path: PathBuf = super::add_special_extension(&filename);
        if path.exists() {
            try!(std::fs::remove_file(&path));
        }

        let mut bytes_written: usize = 0;
        let mut file: File = try!(File::create(path));
        bytes_written += try!(write_magic_number(&mut file));
        bytes_written += try!(write_file_line_offsets(&mut file, &line_offsets));
        bytes_written += try!(write_ngram_array_data(&mut file, &ngram_hash, bytes_written));

        info!("completed index");
        debug!("{} bytes written", bytes_written);
        Ok(())
    }

    fn write_magic_number(file: &mut File) -> std::io::Result<usize> {
        debug!("writing magic number");
        file.write(&BINARY_MAGIC_NUMBER)
    }

    fn write_file_line_offsets(file: &mut File, line_offsets: &[ByteOffset]) -> std::io::Result<usize> {
        let mut total_bytes_written: usize = 0;

        let line_offset_len_bytes: Vec<u8> = util::to_u8_vec(line_offsets.len() as u64);
        total_bytes_written += try!(file.write(&line_offset_len_bytes));
        for line_offset in line_offsets {
            let vec: Vec<u8> = util::to_u8_vec(line_offset);
            total_bytes_written += try!(file.write(&vec));
        }

        debug!("writing newline byte offsets");
        debug!(" --> total lines: {}", line_offsets.len());
        debug!(" --> total bytes: {}", total_bytes_written);

        Ok(total_bytes_written)
    }

    fn write_ngram_array_data(file: &mut File, ngram_hash: &NgramHashMap, start_offset: usize) -> std::io::Result<usize> {
        let mut total_bytes_written: usize = 0;

        let mut hash_key_order: Vec<BinaryNgram> = Vec::new();
        let byte_offset_size = std::mem::size_of::<ByteOffset>();
        let line_number_size = std::mem::size_of::<LineNumber>();
        let byte_length_of_trigram_header = 3 + byte_offset_size + line_number_size;

        let mut ngram_array_start_byte: ByteOffset =
            (ngram_hash.keys().len() * byte_length_of_trigram_header + start_offset) as ByteOffset;

        // first write the headers
        for key in ngram_hash.keys() {
            if let Some(ngram_array) = ngram_hash.get(&key) {
                // write the trigram (3-byte) value
                // TODO assumes little endianness
                let bytes: Vec<u8> = util::to_u8_vec(key);
                let (slice, _) = bytes.split_at(3);
                try!(file.write(&slice));

                debug_assert!(slice.len() == 3);
                total_bytes_written += slice.len();

                // write the byte offset of the array
                let array_offset: Vec<u8> = util::to_u8_vec(ngram_array_start_byte as ByteOffset);
                try!(file.write(&array_offset));
                total_bytes_written += array_offset.len();
                ngram_array_start_byte += (ngram_array.len() * line_number_size) as ByteOffset;

                // now write length of array
                let array_len: Vec<u8> = util::to_u8_vec(bytes.len() as LineNumber);
                try!(file.write(&array_len));
                total_bytes_written += array_len.len();

                hash_key_order.push(*key);
            } else {
                debug_assert!(false);
            }
        }

        let mut num_line_numbers = 0;

        // now write the arrays
        for key in hash_key_order {
            if let Some(ngram_array) = ngram_hash.get(&key) {
                for line_num in ngram_array {
                    let vec = util::to_u8_vec(line_num);
                    try!(file.write(&vec));
                    total_bytes_written += vec.len();
                }

                num_line_numbers += ngram_array.len();
            } else {
                debug_assert!(false);
            }
        }


        debug!("writing ngram array byte offsets");
        debug!(" --> total ngrams: {}", ngram_hash.keys().len());
        debug!(" --> total array elements: {}", num_line_numbers);
        debug!(" --> total bytes written: {}", total_bytes_written);

        Ok(total_bytes_written)
    }
}

pub mod decoder {
    extern crate glob;

    use std;

    use types::*;

    pub fn search(search_str: &str) -> std::io::Result<()> {
        let extension_str = super::add_special_extension(&"./**/*");
        let glob_str = extension_str.as_os_str().to_str().unwrap();
        for file in glob::glob(&glob_str).expect("Failed to read glob pattern") {
            debug!("{:?}", file);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn special_extension_works() {
        let relative_filename = "etc/foo/bar.txt";
        let new_rel_path = super::add_special_extension(&relative_filename);
        assert_eq!("etc/foo/bar.txt.inds", new_rel_path.to_str().unwrap());

        let absolute_filename = "/abc/def/g.gif";
        let new_abs_path = super::add_special_extension(&absolute_filename);
        assert_eq!("/abc/def/g.gif.inds", new_abs_path.to_str().unwrap());

        let no_extension = "a";
        let new_no_ext_path = super::add_special_extension(&no_extension);
        assert_eq!("a.inds", new_no_ext_path.to_str().unwrap());
    }
}
