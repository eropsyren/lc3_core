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

        let char_sequence = "this is a message";

        mem.write_reg(0, 0);
        for (i, c) in char_sequence.chars().enumerate() {
            mem.write(i as u16, c as u16);
        }

        puts(&mut mem, &mut io);

        let expected: Vec<u16> = char_sequence.chars().map(|c| c as u16).collect();
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

        for (expected_val, found_val) in expected.iter().zip(found.iter()) {
            assert_eq!(expected_val, found_val);
        }
    }
}
