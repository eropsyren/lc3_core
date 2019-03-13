use crate::mem::Memory;

pub fn br(instr: u16, mem: &mut Memory) {}

pub fn add(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let imm = (instr >> 5) & 0x1;

    if imm == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);

        let mut val = mem.read_reg(sr1);

        val += imm5;

        mem.set_reg(dr, val);
    } else {
        let sr2 = instr & 0x7;

        let a = mem.read_reg(sr1);
        let b = mem.read_reg(sr2);

        mem.set_reg(dr, a + b);
    }

    mem.update_flags(dr);
}

pub fn ld(instr: u16, mem: &mut Memory) {}

pub fn st(instr: u16, mem: &mut Memory) {}

pub fn jsr(instr: u16, mem: &mut Memory) {}

pub fn and(instr: u16, mem: &mut Memory) {}

pub fn ldr(instr: u16, mem: &mut Memory) {}

pub fn str(instr: u16, mem: &mut Memory) {}

pub fn rti(instr: u16, mem: &mut Memory) {}

pub fn not(instr: u16, mem: &mut Memory) {}

pub fn ldi(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let mem_location = mem.read_pc();

    mem.set_reg(dr, mem_location + pc_offset);

    mem.update_flags(dr);
}

pub fn sti(instr: u16, mem: &mut Memory) {}

pub fn jmp(instr: u16, mem: &mut Memory) {}

pub fn res(instr: u16, mem: &mut Memory) {}

pub fn lea(instr: u16, mem: &mut Memory) {}

pub fn trap(instr: u16, mem: &mut Memory) {}

pub fn sign_extend(x: u16, bit_count: i32) -> u16 {
    let mut x = x;

    if ((x >> (bit_count - 1)) & 1) != 0 {
        x |= 0xFFFF << bit_count;
    }

    x
}
