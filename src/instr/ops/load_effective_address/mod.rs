use super::utils;
use crate::mem::Memory;

pub fn lea(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let pc = mem.read_pc();

    mem.write_reg(dr, pc + pc_offset);

    mem.update_flags(dr);
}
