use std;
use std::vec::Vec;

// MUCH DANGER
// TODO: figure out endianness
pub fn to_u8_vec<T: std::marker::Sized>(in_num: T) -> Vec<u8> {
    let size: usize = std::mem::size_of::<T>();
    let pp: *const T = &in_num;
    let pointer: *const u8 = pp as *const _;
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(pointer, size)
    };

    // quick hack for my little endian machine
    let mut vec = bytes.to_vec();
    vec.reverse();
    vec
}

#[cfg(test)]
mod tests {
    use std;

    #[test]
    fn to_u8_vec_for_usize() {
        let in_num: usize = 0x3269BF19;
        let out_vec = super::to_u8_vec(in_num);

        assert_eq!(out_vec.len(), std::mem::size_of::<usize>());
        let len = out_vec.len();
        assert_eq!(0x32, out_vec[len - 4]);
        assert_eq!(0x69, out_vec[len - 3]);
        assert_eq!(0xBF, out_vec[len - 2]);
        assert_eq!(0x19, out_vec[len - 1]);
    }
}
