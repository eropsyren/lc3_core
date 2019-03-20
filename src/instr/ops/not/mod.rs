use crate::mem::Memory;

pub fn not(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr = (instr >> 6) & 0x7;

    let val = mem.read_reg(sr);

    mem.write_reg(dr, !val);

    mem.update_flags(dr);
}

#[cfg(test)]
mod tests {
    use super::not;
    use crate::mem::Memory;
    use crate::mem::{COND_NEG, COND_POS, COND_ZRO};

    #[test]
    fn test_not() {
        let mut mem = Memory::new();
        let instr = 0b_1001_001_010_1_11111;
        mem.write_reg(2, 0b_0000_1111_0000_1111);
        not(instr, &mut mem);

        assert_eq!(0b_1111_0000_1111_0000, mem.read_reg(1));
        assert_eq!(COND_NEG, mem.read_cond());
    }
}
