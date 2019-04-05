use crate::io::IODevice;
use crate::mem::Memory;

pub fn input(mem: &mut Memory, io: &mut dyn IODevice) {
    io.print_str("Enter a character: ");

    let c = io.get_char();

    mem.write_reg(0, c);
}

#[cfg(tets)]
mod tests {
    use super::input;
    use crate::io::IODevice;
    use crate::io::MockIODevice;
    use crate::io::Msg;
    use crate::io::Val;
    use crate::mem::Memory;

    #[test]
    fn tets_input() {
        let mut mem = Memory::new();
        let mut io = MockIODevice::new();

        input(&mut mem, &mut io);

        assert_eq!(
            Some((
                Msg::PrintStr,
                Val::String(String::from("Enter a character: "))
            )),
            io.logs().first().cloned()
        );

        let c = match io.last() {
            Some(_, Val::U16(c)) => c,
            _ => unreachable!(),
        };

        assert_eq!(c, mem.read_reg(0));
    }
}
