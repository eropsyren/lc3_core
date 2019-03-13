pub fn sign_extend(x: u16, bit_count: i32) -> u16 {
    let mut x = x;

    if ((x >> (bit_count - 1)) & 1) != 0 {
        x |= 0xFFFF << bit_count;
    }

    x
}