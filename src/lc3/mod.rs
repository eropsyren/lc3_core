use crate::controller::Controller;
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
}
