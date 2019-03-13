use super::memory::Memory;

const OP_BR: u16 = 0;
const OP_ADD: u16 = 1;
const OP_LD: u16 = 2;
const OP_ST: u16 = 3;
const OP_JSR: u16 = 4;
const OP_AND: u16 = 5;
const OP_LDR: u16 = 6;
const OP_STR: u16 = 7;
const OP_RTI: u16 = 8;
const OP_NOT: u16 = 9;
const OP_LDI: u16 = 10;
const OP_STI: u16 = 11;
const OP_JMP: u16 = 12;
const OP_RES: u16 = 13;
const OP_LEA: u16 = 14;
const OP_TRAP: u16 = 15;

pub fn exec(instr: u16, mem: &mut Memory) {
    let opcode = instr >> 12;

    match opcode {
        OP_BR => BR(instr, mem),
        OP_ADD => add(instr, mem),
        OP_LD => LD(instr, mem),
        OP_ST => ST(instr, mem),
        OP_JSR => JSR(instr, mem),
        OP_AND => AND(instr, mem),
        OP_LDR => LDR(instr, mem),
        OP_STR => STR(instr, mem),
        OP_RTI => RTI(instr, mem),
        OP_NOT => NOT(instr, mem),
        OP_LDI => ldi(instr, mem),
        OP_STI => STI(instr, mem),
        OP_JMP => JMP(instr, mem),
        OP_RES => RES(instr, mem),
        OP_LEA => LEA(instr, mem),
        OP_TRAP => TRAP(instr, mem),
        _ => unreachable!("Bad instruction opcode: {:b}", opcode),
    }
}

fn BR(instr: u16, mem: &mut Memory) {

}

fn add(instr: u16, mem: &mut Memory) {
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

fn LD(instr: u16, mem: &mut Memory) {

}

fn ST(instr: u16, mem: &mut Memory) {

}

fn JSR(instr: u16, mem: &mut Memory) {

}

fn AND(instr: u16, mem: &mut Memory) {

}

fn LDR(instr: u16, mem: &mut Memory) {

}

fn STR(instr: u16, mem: &mut Memory) {

}

fn RTI(instr: u16, mem: &mut Memory) {

}

fn NOT(instr: u16, mem: &mut Memory) {

}

fn ldi(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let mem_location = mem.read_pc();

    mem.set_reg(dr, mem_location + pc_offset);

    mem.update_flags(dr);
}

fn STI(instr: u16, mem: &mut Memory) {

}

fn JMP(instr: u16, mem: &mut Memory) {

}

fn RES(instr: u16, mem: &mut Memory) {

}

fn LEA(instr: u16, mem: &mut Memory) {

}

fn TRAP(instr: u16, mem: &mut Memory) {

}

fn sign_extend(x: u16, bit_count: i32) -> u16 {
    
    let mut x = x;

    if ((x >> (bit_count - 1)) & 1) != 0 {
        x |= 0xFFFF << bit_count;
    }

    x
}