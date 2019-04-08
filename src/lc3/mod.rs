use crate::controller::Controller;
use crate::instr;
use crate::io::IODevice;
use crate::mem::Memory;

pub struct LC3 {
    memory: Memory,
    io_device: Box<dyn IODevice>,
    controller: Controller,
}

impl LC3 {
    pub fn new_with(io_device: Box<dyn IODevice>) -> LC3 {
        let memory = Memory::new();
        let controller = Controller::new();

        LC3 {
            memory,
            io_device,
            controller,
        }
    }

    pub fn exec(&mut self) {
        let pc = self.memory.read_pc();
        let instr = self.memory.read(pc);

        self.memory.write_pc(pc + 1);

        instr::exec(
            instr,
            &mut self.memory,
            &mut *self.io_device,
            &mut self.controller,
        );
    }
}
