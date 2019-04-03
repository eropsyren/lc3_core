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

pub enum Msg {
    PrintStr,
    PrintChar,
    GetChar,
    Empty,
}

pub enum Val {
    String(String),
    U16(u16),
    Empty,
}
