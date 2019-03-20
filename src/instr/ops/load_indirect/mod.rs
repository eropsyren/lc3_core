use super::utils;
use crate::mem::Memory;

pub fn ldi(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let mem_location = mem.read_pc();
    let mem_location = mem.read(mem_location + pc_offset);
    let val = mem.read(mem_location);

    mem.write_reg(dr, val);

    mem.update_flags(dr);
}

#[cfg(test)]
mod tests {
    use super::ldi;
    use crate::mem::Memory;
    use crate::mem::{COND_NEG, COND_POS, COND_ZRO};

    #[test]
    fn test_ldi() {
        let mut mem = Memory::new();
        let instr = 0b_1010_010_000000011;
        mem.write_pc(1);
        mem.write(4, 100);
        mem.write(100, 0b_1111_1111_1111_1111);
        ldi(instr, &mut mem);

        assert_eq!(-1, mem.read_reg(2) as i16);
        assert_eq!(COND_NEG, mem.read_cond());
    }
}
