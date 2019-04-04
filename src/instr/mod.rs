mod opcode;
mod ops;

use crate::controller::Controller;
use crate::io::IODevice;
use crate::mem::Memory;

pub fn exec(instr: u16, mem: &mut Memory, io: &mut dyn IODevice, ctrl: &mut Controller) {
    let opcode = instr >> 12;

    match opcode {
        opcode::BR => ops::br(instr, mem),
        opcode::ADD => ops::add(instr, mem),
        opcode::LD => ops::ld(instr, mem),
        opcode::ST => ops::st(instr, mem),
        opcode::JSR => ops::jsr(instr, mem),
        opcode::AND => ops::and(instr, mem),
        opcode::LDR => ops::ldr(instr, mem),
        opcode::STR => ops::str(instr, mem),
        opcode::RTI => ops::rti(instr, mem),
        opcode::NOT => ops::not(instr, mem),
        opcode::LDI => ops::ldi(instr, mem),
        opcode::STI => ops::sti(instr, mem),
        opcode::JMP => ops::jmp(instr, mem),
        opcode::RES => ops::res(instr, mem),
        opcode::LEA => ops::lea(instr, mem),
        opcode::TRAP => ops::trap(instr, mem, io, ctrl),
        _ => unreachable!("Bad instruction opcode: {:b}", opcode),
    }
}
