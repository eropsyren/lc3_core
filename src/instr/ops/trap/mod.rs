mod trap_codes;
mod traps;

use crate::io::IODevice;
use crate::mem::Memory;
use crate::controller::Controller;

pub fn trap(instr: u16, mem: &mut Memory, io: Box<dyn IODevice>, ctrl: &mut Controller) {
    let trap_code = instr & 0xFF;

    match trap_code {
        trap_codes::TRAP_GETC => traps::getc(mem, io),
        trap_codes::TRAP_OUT => traps::out(mem, io),
        trap_codes::TRAP_PUTS => traps::puts(mem, io),
        trap_codes::TRAP_IN => traps::input(mem, io),
        trap_codes::TRAP_PUTSP => traps::putsp(mem, io),
        trap_codes::TRAP_HALT => traps::halt(mem, ctrl),
        _ => unreachable!("Bad trap opcode: {:b}", trap_code),
    }
}
