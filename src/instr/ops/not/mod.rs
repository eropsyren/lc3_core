use crate::mem::Memory;

pub fn not(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr = (instr >> 6) & 0x7;

    let val = mem.read_reg(sr);

    mem.write_reg(dr, !val);

    mem.update_flags(dr);
}
