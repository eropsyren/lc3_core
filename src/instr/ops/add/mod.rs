use super::utils;
use crate::mem::Memory;

pub fn add(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let imm = (instr >> 5) & 0x1;

    if imm == 1 {
        let imm5 = utils::sign_extend(instr & 0x1F, 5);

        let mut val = mem.read_reg(sr1);

        val += imm5;

        mem.write_reg(dr, val);
    } else {
        let sr2 = instr & 0x7;

        let a = mem.read_reg(sr1);
        let b = mem.read_reg(sr2);

        mem.write_reg(dr, a + b);
    }

    mem.update_flags(dr);
}

#[cfg(test)]
mod tests {

    use super::add;
    use crate::mem::Memory;
    use crate::mem::{COND_NEG, COND_POS, COND_ZRO};

    #[test]
    fn test_add_immediate() {
        let mut mem = Memory::new();
        let instr = 0b_0001_000_001_1_00001;
        add(instr, &mut mem);

        assert_eq!(1, mem.read_reg(0));
        assert_eq!(COND_POS, mem.read_cond());
    }

    #[test]
    fn test_add_register() {
        let mut mem = Memory::new();
        let instr = 0b_0001_000_001_0_00_010;
        mem.write_reg(1, 1);
        mem.write_reg(2, 1);
        add(instr, &mut mem);

        assert_eq!(2, mem.read_reg(0));
        assert_eq!(COND_POS, mem.read_cond());
    }

    #[test]
    fn test_add_negative() {
        let mut mem = Memory::new();
        let instr = 0b_0001_000_001_1_11111;
        add(instr, &mut mem);

        assert_eq!(-1, mem.read_reg(0) as i16);
        assert_eq!(COND_NEG, mem.read_cond());
    }
}
