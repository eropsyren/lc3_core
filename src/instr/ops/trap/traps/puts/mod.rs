use crate::io::IODevice;
use crate::mem::Memory;

pub fn puts(mem: &mut Memory, io: &mut dyn IODevice) {
    let mut index = mem.read_reg(0);
    let mut c = mem.read(index);

    while c != 0x0000 {
        io.print_char(c);

        index += 1;

        c = mem.read(index);
    }
}

#[cfg(test)]
mod tests {
    use super::puts;
    use crate::io::MockIODevice;
    use crate::io::Msg;
    use crate::io::Val;
    use crate::mem::Memory;

    #[test]
    fn test_puts() {
        let mut mem = Memory::new();
        let mut io = MockIODevice::new();

        let message = "this is a message";
        let expected: Vec<u16> = message.chars().map(|c| c as u16).collect();

        mem.write_reg(0, 0);
        for (i, c) in message.chars().enumerate() {
            mem.write(i as u16, c as u16);
        }

        puts(&mut mem, &mut io);

        let found: Vec<u16> = io
            .logs()
            .iter()
            .map(|log| match log {
                (Msg::PrintChar, v) => v.clone(),
                _ => unreachable!(),
            })
            .map(|val| match val {
                Val::U16(c) => c,
                _ => unreachable!(),
            })
            .collect();

        assert_eq!(expected, found);
    }
}
