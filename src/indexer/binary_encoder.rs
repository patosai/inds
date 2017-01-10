use util;

use std;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use types::*;

static FILE_SUFFIX: &'static str = "inds";
static MAGIC_NUMBER: &'static [u8] = &[0x13, 0x37, 0xBE, 0xEF];

pub fn encode(filename: &str, line_offsets: &[ByteOffset], ngram_hash: &NgramHashMap) -> std::io::Result<()> {
    let path: PathBuf = add_special_extension(&filename);
    if path.exists() {
        try!(std::fs::remove_file(&path));
    }

    let mut bytes_written: usize = 0;
    let mut file: File = try!(File::create(path));
    bytes_written += try!(write_magic_number(&mut file));
    debug!("wrote magic number");
    bytes_written += try!(write_line_offsets(&mut file, &line_offsets));
    debug!("wrote newline byte offsets");
    bytes_written += try!(write_ngram_arrays(&mut file, &ngram_hash));
    debug!("wrote ngram array byte offsets");

    info!("completed encoding index");
    debug!("{} bytes written", bytes_written);
    Ok(())
}

fn add_special_extension(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(&filename);
    let mut extension = path
        .extension()
        .map_or(OsString::new(), |e| {
            let mut e = e.to_os_string();
            e.push(".");
            e
        });
    extension.push(&FILE_SUFFIX);
    path.set_extension(&extension);
    path
}

fn write_magic_number(file: &mut File) -> std::io::Result<usize> {
    file.write(&MAGIC_NUMBER)
}

fn write_line_offsets(file: &mut File, line_offsets: &[ByteOffset]) -> std::io::Result<usize> {
    let mut total_bytes_written: usize = 0;

    let line_offset_len_bytes: Vec<u8> = util::to_u8_vec(line_offsets.len() as u64);
    total_bytes_written += try!(file.write(&line_offset_len_bytes));
    for line_offset in line_offsets {
        let vec: Vec<u8> = util::to_u8_vec(line_offset);
        total_bytes_written += try!(file.write(&vec));
    }

    Ok(total_bytes_written)
}

fn write_ngram_arrays(file: &mut File, ngram_hash: &NgramHashMap) -> std::io::Result<usize> {
    let mut total_bytes_written: usize = 0;

    // 1b. now search for how long the arrays will be and write those values
    for key in ngram_hash.keys() {
        if let Some(lines_for_ngram) = ngram_hash.get(&key) {
            // write the 3-byte value
            // TODO assumes little endianness
            let bytes: Vec<u8> = util::to_u8_vec(key);
            try!(file.write(&bytes[..4]));

            total_bytes_written += 3;

            // now write length of array
            let array_len: Vec<u8> = util::to_u8_vec(bytes.len() as ByteOffset);
            try!(file.write(&array_len));

            total_bytes_written += bytes.len();

            // now write array
            for line_num in lines_for_ngram {
                let vec = util::to_u8_vec(line_num);
                try!(file.write(&vec));
            }

            total_bytes_written += std::mem::size_of::<LineNumber>();
        }
    }

    Ok(total_bytes_written)
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
