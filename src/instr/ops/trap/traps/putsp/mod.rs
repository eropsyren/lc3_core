use crate::io::IODevice;
use crate::mem::Memory;

pub fn putsp(mem: &mut Memory, io: Box<dyn IODevice>) {
    let mut index = mem.read_reg(0);
    let mut c = mem.read(index);

    while c != 0x0000 {
        let c1 = c & 0xFF;

        io.print_char(c1);

        let c2 = c >> 8;

        if c2 != 0x0000 {
            io.print_char(c2)
        };

        index += 1;

        c = mem.read(index);
    }
}
