use crate::io::IODevice;
use crate::mem::Memory;

pub fn getc(mem: &mut Memory, io: &mut Box<dyn IODevice>) {
    mem.write_reg(0, io.get_char());
}
