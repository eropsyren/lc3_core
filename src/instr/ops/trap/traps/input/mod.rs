use crate::io::IODevice;
use crate::mem::Memory;

pub fn input(mem: &mut Memory, io: &mut dyn IODevice) {
    io.print_str("Enter a character: ");

    let c = io.get_char();

    mem.write_reg(0, c);
}
