use crate::io::IODevice;
use crate::mem::Memory;

pub fn putsp(mem: &mut Memory, io: &mut dyn IODevice) {
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

#[cfg(test)]
mod tests {
    use super::putsp;
    use crate::io::MockIODevice;
    use crate::io::Msg;
    use crate::io::Val;
    use crate::mem::Memory;

    #[test]
    fn test_putsp() {
        let mut mem = Memory::new();
        let mut io = MockIODevice::new();

        let message = vec![
            't' as u16 | (('h' as u16) << 8),
            'i' as u16 | (('s' as u16) << 8),
            ' ' as u16 | (('i' as u16) << 8),
            's' as u16 | ((' ' as u16) << 8),
            'a' as u16 | ((' ' as u16) << 8),
            'm' as u16 | (('e' as u16) << 8),
            's' as u16 | (('s' as u16) << 8),
            'a' as u16 | (('g' as u16) << 8),
            'e' as u16,
        ];

        let expected: Vec<u16> = "this is a message".chars().map(|c| c as u16).collect();

        mem.write_reg(0, 0);
        for (i, c) in message.iter().enumerate() {
            mem.write(i as u16, *c);
        }

        putsp(&mut mem, &mut io);

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
