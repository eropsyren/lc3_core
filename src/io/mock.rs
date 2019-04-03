use super::IODevice;

const a_char: u16 = 97;

pub struct MockIODevice {
    msg: Option<Msg>,
    val: Option<Val>,
}

impl MockIODevice {
    pub fn new() -> MockIODevice {
        MockIODevice {
            msg: None,
            val: None,
        }
    }

    pub fn get_msg(&self) -> &Option<Msg> {
        &self.msg
    }

    pub fn get_val(&self) -> &Option<Val> {
        &self.val
    }
}

impl IODevice for MockIODevice {
    fn print_str(&mut self, str: &str) {
        self.msg = Some(Msg::PrintStr);
        self.val = Some(Val::String(String::from(str)));
    }

    fn print_char(&mut self, c: u16) {
        self.msg = Some(Msg::PrintChar);
        self.val = Some(Val::U16(c));
    }

    fn get_char(&mut self) -> u16 {
        self.msg = Some(Msg::GetChar);
        self.val = Some(Val::U16(a_char));

        a_char
    }
}

impl PartialEq for MockIODevice {
    fn eq(&self, other: &MockIODevice) -> bool {
        self.msg == other.msg && self.val == other.val
    }
}

#[derive(PartialEq, Debug)]
pub enum Msg {
    PrintStr,
    PrintChar,
    GetChar,
}

#[derive(PartialEq, Debug)]
pub enum Val {
    String(String),
    U16(u16),
}

#[cfg(test)]
mod tests {
    use super::MockIODevice;
    use super::Msg;
    use super::Val;
    use crate::IODevice;

    #[test]
    fn test_new() {
        let io = MockIODevice::new();

        assert_eq!(io.msg, None);
        assert_eq!(io.val, None);
    }

    #[test]
    fn test_get_msg() {
        let mut io = MockIODevice::new();
        io.msg = Some(Msg::GetChar);

        assert_eq!(Some(Msg::GetChar), *io.get_msg());
    }

    #[test]
    fn test_get_val() {
        let mut io = MockIODevice::new();
        io.val = Some(Val::U16(11));

        assert_eq!(Some(Val::U16(11)), *io.get_val());
    }

    #[test]
    fn test_print_str() {
        let mut io = MockIODevice::new();
        io.print_str("This is a &str");

        assert_eq!(Some(Msg::PrintStr), *io.get_msg());
        assert_eq!(
            Some(Val::String(String::from("This is a &str"))),
            *io.get_val()
        );
    }

    #[test]
    fn test_print_char() {
        let mut io = MockIODevice::new();
        io.print_char(11);

        assert_eq!(Some(Msg::PrintChar), *io.get_msg());
        assert_eq!(Some(Val::U16(11)), *io.get_val());
    }

    #[test]
    fn test_get_char() {
        let mut io = MockIODevice::new();
        io.get_char();

        assert_eq!(Some(Msg::GetChar), *io.get_msg());
    }
}
