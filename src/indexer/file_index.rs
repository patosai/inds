use std::collections::HashMap;
use std::vec::Vec;

pub struct FileIndex {
    filename: String,
    hash: HashMap<u32, Vec<u64>>,
}
