use crate::mem::Memory;

// TODO add ret: (return from subroutine) implementation
pub fn jmp(instr: u16, mem: &mut Memory) {
    let base_r = (instr >> 6) & 0x7;

    let val = mem.read_reg(base_r);

    mem.write_pc(val);
}
