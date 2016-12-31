use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;

use types::*;

static FILE_SUFFIX: &'static str = ".inds";

pub fn encode(filename: &str, ngram_hash: &NgramHashMap) {
    let mut path = PathBuf::from(&filename);
    let mut new_extension = path.extension().unwrap_or(OsStr::new("")).to_os_string();
    new_extension.push(&FILE_SUFFIX);
    if path.set_extension(&new_extension) {
        info!("{:?}", path.to_str().unwrap());
        File::create(path);
    }
}
