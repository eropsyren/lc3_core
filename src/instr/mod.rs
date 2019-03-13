mod op;
mod opcode;

use crate::mem::Memory;
use op::*;

pub fn exec(instr: u16, mem: &mut Memory) {
    let opcode = instr >> 12;

    match opcode {
        OP_BR => op::br(instr, mem),
        OP_ADD => op::add(instr, mem),
        OP_LD => op::ld(instr, mem),
        OP_ST => op::st(instr, mem),
        OP_JSR => op::jsr(instr, mem),
        OP_AND => op::and(instr, mem),
        OP_LDR => op::ldr(instr, mem),
        OP_STR => op::str(instr, mem),
        OP_RTI => op::rti(instr, mem),
        OP_NOT => op::not(instr, mem),
        OP_LDI => op::ldi(instr, mem),
        OP_STI => op::sti(instr, mem),
        OP_JMP => op::jmp(instr, mem),
        OP_RES => op::res(instr, mem),
        OP_LEA => op::lea(instr, mem),
        OP_TRAP => op::trap(instr, mem),
        _ => unreachable!("Bad instruction opcode: {:b}", opcode),
    }
}
