use crate::mem::Memory;
use crate::io::IODevice;

pub fn out(mem: &mut Memory, io: Box<dyn IODevice>) {
    let c = mem.read_reg(0);
    
    io.print_char(c);
}