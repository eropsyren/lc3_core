use crate::mem::Memory;
use crate::io::IODevice;

pub fn getc(mem: &mut Memory, io: Box<dyn IODevice>) {
    mem.write_reg(0, io.get_char());
}