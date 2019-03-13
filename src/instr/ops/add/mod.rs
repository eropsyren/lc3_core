use crate::mem::Memory;
use super::utils;

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
