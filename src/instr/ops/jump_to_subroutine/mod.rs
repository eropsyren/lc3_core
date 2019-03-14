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
