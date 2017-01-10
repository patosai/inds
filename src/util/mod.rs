use std;
use std::vec::Vec;

// TODO: figure out endianness
pub fn to_u8_vec<T: std::marker::Sized>(in_num: T) -> Vec<u8> {
    let size: usize = std::mem::size_of::<T>();
    let pp: *const T = &in_num;
    let pointer: *const u8 = pp as *const _;
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(pointer, size)
    };
    bytes.to_vec()
}

// TODO same endian issue
pub fn from_u8_vec<T: std::clone::Clone + std::marker::Sized>(in_vec: &[u8]) -> Vec<T> {
    let size: usize = std::mem::size_of::<T>();
    let num_elems: usize = in_vec.len() / size;
    let pp: *const u8 = unsafe {
        in_vec.get_unchecked(0)
    };
    let pointer: *const T = pp as *const _;
    let bytes: &[T] = unsafe {
        std::slice::from_raw_parts(pointer, num_elems)
    };
    bytes.to_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_vec_is_left_aligned_on_little_endian_machines() {
        let in_num: u32 = 0x0000ABCD;
        let out_vec: Vec<u8> = super::to_u8_vec(in_num);
        assert_eq!(4, out_vec.len());
        assert_eq!(out_vec.get(0).unwrap(), &(0xCD as u8));
        assert_eq!(out_vec.get(1).unwrap(), &(0xAB as u8));
        assert_eq!(out_vec.get(2).unwrap(), &(0x00 as u8));
        assert_eq!(out_vec.get(3).unwrap(), &(0x00 as u8));
    }

    #[test]
    fn to_and_from_vec_for_usize() {
        let in_num: usize = 0x3269BF19;
        let out_vec: Vec<usize> = super::from_u8_vec(&super::to_u8_vec(in_num));
        assert_eq!(1, out_vec.len());
        assert_eq!(&in_num, out_vec.first().unwrap());
    }

    #[test]
    fn to_and_from_vec_for_u64() {
        let in_num: u64 = 0xFEEF1337DEADBEEF;
        let out_vec: Vec<u64> = super::from_u8_vec(&super::to_u8_vec(in_num));
        assert_eq!(1, out_vec.len());
        assert_eq!(&in_num, out_vec.first().unwrap());
    }
}
