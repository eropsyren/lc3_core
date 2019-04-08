use super::utils;
use crate::mem::Memory;

pub fn br(instr: u16, mem: &mut Memory) {
    let cond_flag = (instr >> 9) & 0x7;
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    if cond_flag & mem.read_cond() != 0 {
        let pc = mem.read_pc();

        mem.write_pc(pc + pc_offset);
    }
}

#[cfg(test)]
mod tests {
    use super::br;
    use crate::mem::Memory;

    #[test]
    fn test_branch_zero() {
        let mut mem = Memory::new();
        let instr = 0b_0000_010_000000011;
        mem.write_pc(0);
        br(instr, &mut mem);

        assert_eq!(3, mem.read_pc());
    }

    #[test]
    fn test_branch_pos() {
        let mut mem = Memory::new();
        let instr = 0b_0000_001_000000011;
        mem.write_pc(0);
        mem.write_reg(0, 1);
        mem.update_flags(0);
        br(instr, &mut mem);

        assert_eq!(3, mem.read_pc());
    }

    #[test]
    fn test_branch_neg() {
        let mut mem = Memory::new();
        let instr = 0b_0000_100_000000011;
        mem.write_pc(0);
        mem.write_reg(0, 0b_1000_0000_0000_0001);
        mem.update_flags(0);
        br(instr, &mut mem);

        assert_eq!(3, mem.read_pc());
    }
}
