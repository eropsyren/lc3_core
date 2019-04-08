use super::utils;
use crate::mem::Memory;

pub fn lea(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let pc = mem.read_pc();

    mem.write_reg(dr, pc + pc_offset);

    mem.update_flags(dr);
}

#[cfg(test)]
mod tests {
    use super::lea;
    use crate::mem::Memory;
    use crate::mem::COND_POS;

    #[test]
    fn test_lea() {
        let mut mem = Memory::new();
        let instr = 0b_1110_001_000000011;
        mem.write_pc(9);
        lea(instr, &mut mem);

        assert_eq!(9 + 3, mem.read_reg(1));
        assert_eq!(COND_POS, mem.read_cond());
    }
}
