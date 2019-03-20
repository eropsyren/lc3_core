use super::utils;
use crate::mem::Memory;

pub fn ld(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let pc = mem.read_pc();
    let val = mem.read(pc + pc_offset);

    mem.write_reg(dr, val);

    mem.update_flags(dr);
}

#[cfg(test)]
mod tests {
    use super::ld;
    use crate::mem::Memory;
    use crate::mem::{COND_NEG, COND_POS, COND_ZRO};

    #[test]
    fn test_ld() {
        let mut mem = Memory::new();
        let instr = 0b_0010_110_000000011;
        mem.write(3, 125);
        mem.write_pc(0);
        ld(instr, &mut mem);

        assert_eq!(125, mem.read_reg(6));
        assert_eq!(COND_POS, mem.read_cond());
    }
}
