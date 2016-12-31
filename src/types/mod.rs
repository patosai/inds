use std::collections::HashMap;

pub type BinaryNgram = u32;
pub type ByteOffset = u64;
pub type NgramHashMap = HashMap<BinaryNgram, Vec<ByteOffset>>;
