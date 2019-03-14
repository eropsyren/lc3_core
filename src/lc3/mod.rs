use crate::mem::Memory;
use crate::io::IODevice;
use crate::controller::Controller;

pub struct LC3 {
    memory: Memory,
    io_device: Box<dyn IODevice>,
    controller: Controller,
}