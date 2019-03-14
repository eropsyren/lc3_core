use crate::io::IODevice;
use crate::mem::Memory;

pub fn out(mem: &mut Memory, io: Box<dyn IODevice>) {
    let c = mem.read_reg(0);

    io.print_char(c);
}
