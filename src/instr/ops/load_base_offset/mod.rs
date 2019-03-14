use super::utils;
use crate::mem::Memory;

pub fn ldr(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let base_r = (instr >> 6) & 0x7;
    let offset = utils::sign_extend(instr & 0x3F, 6);

    let mem_location = mem.read_reg(base_r);
    let val = mem.read(mem_location + offset);

    mem.write_reg(dr, val);

    mem.update_flags(dr);
}
