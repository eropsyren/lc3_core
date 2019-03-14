use super::utils;
use crate::mem::Memory;

pub fn st(instr: u16, mem: &mut Memory) {
    let sr = (instr >> 9) & 0x7;
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let pc = mem.read_pc();
    let val = mem.read_reg(sr);

    mem.write(pc + pc_offset, val);
}
