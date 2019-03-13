mod flags;
mod reg;

use flags::{COND_NEG, COND_POS, COND_ZRO};
use reg::Register;

const CELL_COUNT: usize = 65_536;

pub struct Memory {
    cells: [u16; CELL_COUNT],
    registers: [Register; 8],
    pc: Register,
    cond: Register,
}

impl Memory {

    pub fn read_reg(&self, reg: u16) -> u16 {
        let reg = reg as usize;

        match reg {
            0...7 => self.registers[reg].get_val(),
            _ => panic!("Cannot acess r{}", reg),
        }
    }

    pub fn set_reg(&mut self, reg: u16, val: u16) {
        let reg = reg as usize;

        match reg {
            0...7 => self.registers[reg].set_val(val),
            _ => panic!("Cannot acess r{}", reg),
        }
    }

    pub fn read_pc(&self) -> u16 {
        self.pc.get_val()
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc.set_val(val)
    } 

    pub fn update_flags(&mut self, reg: u16) {

        let reg = reg as usize;

        if self.registers.len() >= reg {
            panic!("Cannot acess r{}", reg);
        }

        match self.registers[reg].get_val() {
            0 => self.cond.set_val(COND_ZRO),
            val if val >> 15 == 1 => self.cond.set_val(COND_NEG),
            _ => self.cond.set_val(COND_POS),
        }
    }
}
