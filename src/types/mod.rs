use std::collections::HashMap;

// trigram - 3 bytes - 24 bits
pub type BinaryNgram = u32;

// bytes in a file - a lot
pub type ByteOffset = u64;

// lines in a file - quite a few
// u16 = 65,535 max lines in a file
pub type LineNumber = u16;

pub type NgramHashMap = HashMap<BinaryNgram, Vec<LineNumber>>;
