use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::path::PathBuf;

use types::*;

static FILE_SUFFIX: &'static str = "inds";

pub fn encode(filename: &str, ngram_hash: &NgramHashMap) -> Result<(), Box<Error>> {
    let path = add_extra_extension(&filename);
    let mut file = try!(File::create(path));
    Ok(())
}

fn add_extra_extension(filename: &str) -> PathBuf {
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

#[cfg(test)]
mod tests {
    #[test]
    fn can_add_extension() {
        let relative_filename = "etc/foo/bar.txt";
        let new_rel_path = super::add_extra_extension(&relative_filename);
        assert_eq!("etc/foo/bar.txt.inds", new_rel_path.to_str().unwrap());

        let absolute_filename = "/abc/def/g.gif";
        let new_abs_path = super::add_extra_extension(&absolute_filename);
        assert_eq!("/abc/def/g.gif.inds", new_abs_path.to_str().unwrap());

        let no_extension = "a";
        let new_no_ext_path = super::add_extra_extension(&no_extension);
        assert_eq!("a.inds", new_no_ext_path.to_str().unwrap());
    }
}
