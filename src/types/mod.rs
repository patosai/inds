use std::collections::HashMap;

pub static BINARY_FILE_SUFFIX: &'static str = "inds";
pub static BINARY_MAGIC_NUMBER: &'static [u8] = &[0x13, 0x37, 0xBE, 0xEF];

// trigram - 3 bytes - 24 bits
pub type BinaryNgram = u32;

// bytes in a file - a lot
pub type ByteOffset = u64;

// lines in a file - quite a few
// u16 = 65,535 max lines in a file
pub type LineNumber = u16;

pub type NgramHashMap = HashMap<BinaryNgram, Vec<LineNumber>>;
