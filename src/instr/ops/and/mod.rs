use super::utils;
use crate::mem::Memory;

pub fn and(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = utils::sign_extend(instr & 0x1F, 5);

        let val = mem.read_reg(sr1);

        mem.write_reg(dr, val & imm5);
    } else {
        let sr2 = instr & 0x7;

        let a = mem.read_reg(sr1);
        let b = mem.read_reg(sr2);

        mem.write_reg(dr, a & b);
    }

    mem.update_flags(dr);
}

#[cfg(test)]
mod tests {
    use super::and;
    use crate::mem::Memory;
    use crate::mem::COND_POS;

    #[test]
    fn test_and_immediate() {
        let mut mem = Memory::new();
        let instr = 0b_0101_000_001_1_00001;
        mem.write_reg(1, 1);
        and(instr, &mut mem);

        assert_eq!(1, mem.read_reg(0));
        assert_eq!(COND_POS, mem.read_cond());
    }

    #[test]
    fn test_and_register() {
        let mut mem = Memory::new();
        let instr = 0b_0101_000_001_0_00_010;
        mem.write_reg(1, 1);
        mem.write_reg(2, 3);
        and(instr, &mut mem);

        assert_eq!(1, mem.read_reg(0));
        assert_eq!(COND_POS, mem.read_cond());
    }
}
