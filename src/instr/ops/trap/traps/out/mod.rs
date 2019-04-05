use crate::io::IODevice;
use crate::mem::Memory;

pub fn out(mem: &mut Memory, io: &mut dyn IODevice) {
    let c = mem.read_reg(0);

    io.print_char(c);
}

#[cfg(test)]
mod tests {
    use super::out;
    use crate::io::MockIODevice;
    use crate::io::Msg;
    use crate::io::Val;
    use crate::mem::Memory;

    #[test]
    fn test_out() {
        let mut mem = Memory::new();
        let mut io = MockIODevice::new();

        out(&mut mem, &mut io);

        let c = match io.last() {
            Some((Msg::PrintChar, Val::U16(c))) => c,
            _ => unreachable!(),
        };

        assert_eq!(mem.read_reg(0), c);
    }
}
