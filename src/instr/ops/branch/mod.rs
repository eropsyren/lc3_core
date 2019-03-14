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
