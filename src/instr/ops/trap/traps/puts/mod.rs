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

        puts(&mut mem, &mut io);

        let expected_values: Vec<u16> = io
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

        let mut found_values: Vec<u16> = vec![];

        let mut index = mem.read_reg(0);
        let mut val = mem.read(index);

        while val != 0x0000 {
            found_values.push(val);

            index += 1;

            val = mem.read(index);
        }

        for (expected, found) in expected_values.iter().zip(found_values.iter()) {
            assert_eq!(expected, found);
        }
    }
}
