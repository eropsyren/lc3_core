use crate::mem::Memory;
use super::utils;

pub fn sti(instr: u16, mem: &mut Memory) {
    let sr = (instr >> 9) & 0x7;
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let pc = mem.read_pc();
    let mem_location = mem.read(pc + pc_offset);
    let val = mem.read_reg(sr);

    mem.write(mem_location, val);
}
