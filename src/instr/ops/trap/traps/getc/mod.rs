use crate::io::IODevice;
use crate::mem::Memory;

pub fn getc(mem: &mut Memory, io: &mut dyn IODevice) {
    mem.write_reg(0, io.get_char());
}

#[cfg(test)]
mod tests {
    use super::getc;
    use crate::io::MockIODevice;
    use crate::io::Val;
    use crate::mem::Memory;

    #[test]
    fn test_getc() {
        let mut mem = Memory::new();
        let mut io = MockIODevice::new();

        getc(&mut mem, &mut io);

        let val = match io.get_val() {
            Some(Val::U16(c)) => *c,
            _ => unreachable!(),
        };

        assert_eq!(val, mem.read_reg(0));
    }
}
