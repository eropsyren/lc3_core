use super::IODevice;

const a_char: u16 = 97;

pub struct MockIODevice {
    logs: Vec<(Msg, Val)>,
}

impl MockIODevice {
    pub fn new() -> MockIODevice {
        MockIODevice { logs: vec![] }
    }

    pub fn logs(&self) -> &Vec<(Msg, Val)> {
        &self.logs
    }

    pub fn last(&self) -> Option<(Msg, Val)> {
        let last = self.logs.last();

        match last {
            Some((m, v)) => Some((m.clone(), v.clone())),
            None => None,
        }
    }
}

impl IODevice for MockIODevice {
    fn print_str(&mut self, str: &str) {
        self.logs
            .push((Msg::PrintStr, Val::String(String::from(str))));
    }

    fn print_char(&mut self, c: u16) {
        self.logs.push((Msg::PrintChar, Val::U16(c)));
    }

    fn get_char(&mut self) -> u16 {
        self.logs.push((Msg::GetChar, Val::U16(a_char)));

        a_char
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Msg {
    PrintStr,
    PrintChar,
    GetChar,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Val {
    String(String),
    U16(u16),
}

#[cfg(test)]
mod tests {
    use super::a_char;
    use super::MockIODevice;
    use super::Msg;
    use super::Val;
    use crate::IODevice;

    #[test]
    fn test_new() {
        let io = MockIODevice::new();
        let v: Vec<(Msg, Val)> = vec![];

        assert_eq!(v, io.logs);
    }

    #[test]
    fn test_logs() {
        let io = MockIODevice::new();
        let v: Vec<(Msg, Val)> = vec![
            (Msg::PrintStr, Val::String(String::from("s"))),
            (Msg::PrintChar, Val::U16(0)),
            (Msg::GetChar, Val::U16(a_char)),
        ];

        io.print_str("s");
        io.print_char(0);
        io.get_char();

        for (expected, found) in v.iter().zip(io.logs().iter()) {
            assert_eq!(expected, found);
        }
    }

    #[test]
    fn last() {
        let io = MockIODevice::new();
        let v: Vec<(Msg, Val)> = vec![
            (Msg::PrintStr, Val::String(String::from("s"))),
            (Msg::PrintChar, Val::U16(0)),
            (Msg::GetChar, Val::U16(a_char)),
        ];

        io.print_str("s");
        io.print_char(0);
        io.get_char();

        assert_eq!(Some((Msg::GetChar, Val::U16(a_char))), io.last());
    }

    #[test]
    fn test_print_str() {
        let mut io = MockIODevice::new();
        io.print_str("This is a &str");

        assert_eq!(
            Some((Msg::PrintStr, Val::String(String::from("This is a &str")))),
            io.last()
        );
    }

    #[test]
    fn test_print_char() {
        let mut io = MockIODevice::new();
        io.print_char(11);

        assert_eq!(Some((Msg::PrintChar, Val::U16(11))), io.last());
    }

    #[test]
    fn test_get_char() {
        let mut io = MockIODevice::new();
        io.get_char();

        assert_eq!(Some((Msg::GetChar, Val::U16(a_char))), io.last());
    }
}
