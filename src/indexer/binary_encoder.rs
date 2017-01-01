use util;

use std;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use types::*;

static FILE_SUFFIX: &'static str = "inds";
static MAGIC_NUMBER: &'static [u8] = &[0xBE, 0xEF];

pub fn encode(filename: &str, line_offsets: &[ByteOffset], ngram_hash: &NgramHashMap) -> std::io::Result<()> {
    let path: PathBuf = add_special_extension(&filename);
    if path.exists() {
        try!(std::fs::remove_file(&path));
    }

    let mut bytes_written: usize = 0;
    let mut file: File = try!(File::create(path));
    bytes_written += try!(write_magic_number(&mut file));
    bytes_written += try!(write_line_offsets(&mut file, &line_offsets));

    // NgramHash occupies the lower 24 bits of the u32
    for i in 0..0x1000000 {

    }

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

    let offset_length_bytes = util::to_u8_vec(line_offsets.len() as u64);
    total_bytes_written += try!(file.write(&offset_length_bytes));
    for line_offset in line_offsets {
        let vec = util::to_u8_vec(line_offset);
        total_bytes_written += try!(file.write(&vec));
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
