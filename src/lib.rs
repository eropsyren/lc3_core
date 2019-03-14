mod controller;
mod instr;
mod io;
mod mem;

use io::IODevice;
use mem::Memory;
use controller::Controller;

pub struct LC3 {
    memory: Memory,
    io_device: Box<dyn IODevice>,
    controller: Controller,
}
