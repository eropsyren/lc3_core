use crate::mem::Memory;

pub fn br(instr: u16, mem: &mut Memory) {
    let cond_flag = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    if cond_flag & mem.read_cond() != 0 {
        let pc = mem.read_pc();

        mem.write_pc(pc + pc_offset);
    }
}

pub fn add(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let imm = (instr >> 5) & 0x1;

    if imm == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);

        let mut val = mem.read_reg(sr1);

        val += imm5;

        mem.write_reg(dr, val);
    } else {
        let sr2 = instr & 0x7;

        let a = mem.read_reg(sr1);
        let b = mem.read_reg(sr2);

        mem.write_reg(dr, a + b);
    }

    mem.update_flags(dr);
}

pub fn ld(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let pc = mem.read_pc();
    let val = mem.read(pc + pc_offset);

    mem.write_reg(dr, val);

    mem.update_flags(dr);
}

pub fn st(instr: u16, mem: &mut Memory) {
    let sr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let pc = mem.read_pc();
    let val = mem.read_reg(sr);

    mem.write(pc + pc_offset, val);
}

pub fn jsr(instr: u16, mem: &mut Memory) {
    let long_flag = (instr >> 11) & 1;

    let pc = mem.read_pc();

    mem.write_reg(7, pc);

    if long_flag == 1 {
        let pc_offset = sign_extend(instr & 0x7ff, 11);

        mem.write_pc(pc + pc_offset);
    } else {
        let base_r = (instr >> 6) & 0x7;

        let val = mem.read_reg(base_r);

        mem.write_pc(val);
    }
}

pub fn and(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);

        let val = mem.read_reg(sr1);

        mem.write_reg(dr, val & imm5);
    } else {
        let sr2 = instr & 0x7;

        let a = mem.read_reg(sr1);
        let b = mem.read_reg(sr2);

        mem.write_reg(dr, a & b);
    }

    mem.update_flags(dr);
}

pub fn ldr(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let base_r = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);

    let mem_location = mem.read_reg(base_r);
    let val = mem.read(mem_location + offset);

    mem.write_reg(dr, val);

    mem.update_flags(dr);
}

pub fn str(instr: u16, mem: &mut Memory) {
    let sr = (instr >> 9) & 0x7;
    let base_r = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);

    let mem_location = mem.read_reg(base_r);
    let val = mem.read_reg(sr);

    mem.write(mem_location + offset, val);
}

pub fn rti(instr: u16, mem: &mut Memory) {
    panic!("RTI instruction is unused");
}

pub fn not(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let sr = (instr >> 6) & 0x7;

    let val = mem.read_reg(sr);

    mem.write_reg(dr, !val);

    mem.update_flags(dr);
}

pub fn ldi(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let mem_location = mem.read_pc();
    let mem_location = mem.read(mem_location + pc_offset);
    let val = mem.read(mem_location);

    mem.write_reg(dr, val);

    mem.update_flags(dr);
}

pub fn sti(instr: u16, mem: &mut Memory) {
    let sr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let pc = mem.read_pc();
    let mem_location = mem.read(pc + pc_offset);
    let val = mem.read_reg(sr);

    mem.write(mem_location, val);
}

pub fn jmp(instr: u16, mem: &mut Memory) {
    let base_r = (instr >> 6) & 0x7;

    let val = mem.read_reg(base_r);

    mem.write_pc(val);
}

pub fn res(instr: u16, mem: &mut Memory) {
    panic!("RES instruction is unused");
}

pub fn lea(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let pc = mem.read_pc();

    mem.write_reg(dr, pc + pc_offset);

    mem.update_flags(dr);
}

pub fn trap(instr: u16, mem: &mut Memory) {}

pub fn sign_extend(x: u16, bit_count: i32) -> u16 {
    let mut x = x;

    if ((x >> (bit_count - 1)) & 1) != 0 {
        x |= 0xFFFF << bit_count;
    }

    x
}
