use super::utils;
use crate::mem::Memory;

pub fn jsr(instr: u16, mem: &mut Memory) {
    let long_flag = (instr >> 11) & 1;

    let pc = mem.read_pc();

    mem.write_reg(7, pc);

    if long_flag == 1 {
        let pc_offset = utils::sign_extend(instr & 0x7ff, 11);

        mem.write_pc(pc + pc_offset);
    } else {
        let base_r = (instr >> 6) & 0x7;

        let val = mem.read_reg(base_r);

        mem.write_pc(val);
    }
}

#[cfg(test)]
mod tests {
    use super::jsr;
    use crate::mem::Memory;

    #[test]
    fn test_jsr() {
        let mut mem = Memory::new();
        let instr = 0b_0100_1_00000000011;
        mem.write_pc(255);
        jsr(instr, &mut mem);

        assert_eq!(255, mem.read_reg(7));
        assert_eq!(255 + 3, mem.read_pc());
    }

    #[test]
    fn test_jsrr() {
        let mut mem = Memory::new();
        let instr = 0b_0100_0_00_001_000000;
        mem.write_reg(1, 4000);
        mem.write_pc(255);
        jsr(instr, &mut mem);

        assert_eq!(255, mem.read_reg(7));
        assert_eq!(4000, mem.read_pc());
    }
}
