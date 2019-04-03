use crate::io::IODevice;
use crate::mem::Memory;

pub fn getc(mem: &mut Memory, mut io: Box<dyn IODevice>) {
    mem.write_reg(0, io.get_char());
}
