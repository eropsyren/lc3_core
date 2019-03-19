mod flags;
mod reg;

use flags::{COND_NEG, COND_POS, COND_ZRO};
use reg::Register;

const CELL_COUNT: usize = 65_536;
const PC_START: u16 = 0x3000;

pub struct Memory {
    cells: [u16; CELL_COUNT],
    registers: [Register; 8],
    pc: Register,
    cond: Register,
}

impl Memory {
    pub fn new() -> Memory {
        let cells = [0; CELL_COUNT];
        let registers = [Register::new(); 8];
        let pc = Register::with(PC_START);
        let cond = Register::with(COND_ZRO);

        Memory {
            cells,
            registers,
            pc,
            cond,
        }
    }

    pub fn read(&self, cell: u16) -> u16 {
        self.cells[cell as usize]
    }

    pub fn write(&mut self, cell: u16, val: u16) {
        self.cells[cell as usize] = val;
    }

    pub fn read_reg(&self, reg: u16) -> u16 {
        let reg = reg as usize;

        match reg {
            0...7 => self.registers[reg].get(),
            _ => panic!("Cannot read r{}", reg),
        }
    }

    pub fn write_reg(&mut self, reg: u16, val: u16) {
        let reg = reg as usize;

        match reg {
            0...7 => self.registers[reg].set(val),
            _ => panic!("Cannot write r{}", reg),
        }
    }

    pub fn read_pc(&self) -> u16 {
        self.pc.get()
    }

    pub fn write_pc(&mut self, val: u16) {
        self.pc.set(val)
    }

    pub fn read_cond(&self) -> u16 {
        self.cond.get()
    }

    pub fn update_flags(&mut self, reg: u16) {
        let reg = reg as usize;

        if reg >= self.registers.len() {
            panic!("Cannot acess r{}", reg);
        }

        match self.registers[reg].get() {
            0 => self.cond.set(COND_ZRO),
            val if val >> 15 == 1 => self.cond.set(COND_NEG),
            _ => self.cond.set(COND_POS),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::flags::{COND_NEG, COND_POS, COND_ZRO};
    use super::Memory;

    #[test]
    fn test_read() {
        let mut mem = Memory::new();
        mem.cells[10] = 1;

        assert_eq!(1, mem.read(10));
    }

    #[test]
    fn test_write() {
        let mut mem = Memory::new();
        mem.write(0, 1);

        assert_eq!(1, mem.cells[0]);
    }

    #[test]
    fn test_read_reg() {
        let mut mem = Memory::new();
        mem.registers[0].set(255);

        assert_eq!(255, mem.read_reg(0));
    }

    #[test]
    fn test_write_reg() {
        let mut mem = Memory::new();
        mem.write_reg(7, 200);

        assert_eq!(200, mem.registers[7].get());
    }

    #[test]
    fn test_read_pc() {
        let mut mem = Memory::new();
        mem.pc.set(0x3000);

        assert_eq!(0x3000, mem.read_pc());
    }

    #[test]
    fn test_write_pc() {
        let mut mem = Memory::new();
        mem.write_pc(800);

        assert_eq!(800, mem.pc.get());
    }

    #[test]
    fn test_read_cond() {
        let mut mem = Memory::new();
        mem.cond.set(COND_NEG);

        assert_eq!(COND_NEG, mem.read_cond());
    }

    #[test]
    fn test_update_flags_zro() {
        let mut mem = Memory::new();
        mem.write_reg(0, 0);
        mem.update_flags(0);

        assert_eq!(COND_ZRO, mem.read_cond());
    }

    #[test]
    fn test_update_flags_pos() {
        let mut mem = Memory::new();
        mem.write_reg(0, 1);
        mem.update_flags(0);

        assert_eq!(COND_POS, mem.read_cond());
    }

    #[test]
    fn test_update_flags_neg() {
        let mut mem = Memory::new();
        mem.write_reg(0, 0b1000_0000_0000_0001);
        mem.update_flags(0);

        assert_eq!(COND_NEG, mem.read_cond());
    }
}
