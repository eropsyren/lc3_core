use crate::io::IODevice;
use crate::mem::Memory;

pub fn puts(mem: &mut Memory, mut io: Box<dyn IODevice>) {
    let mut index = mem.read_reg(0);
    let mut c = mem.read(index);

    while c != 0x0000 {
        io.print_char(c);

        index += 1;

        c = mem.read(index);
    }
}
