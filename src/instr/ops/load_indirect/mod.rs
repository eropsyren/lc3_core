use crate::mem::Memory;
use super::utils;

pub fn ldi(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let mem_location = mem.read_pc();
    let mem_location = mem.read(mem_location + pc_offset);
    let val = mem.read(mem_location);

    mem.write_reg(dr, val);

    mem.update_flags(dr);
}
