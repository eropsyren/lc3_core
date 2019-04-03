use super::IODevice;

const a_char: u16 = 97;

pub struct MockIODevice {
    msg: Msg,
}

impl IODevice for MockIODevice {
    fn print_str(&mut self, str: &str) {
        self.msg = Msg::PrintStr(String::from(str));
    }

    fn print_char(&mut self, c: u16) {
        self.msg = Msg::PrintChar(c);
    }

    fn get_char(&mut self) -> u16 {
        self.msg = Msg::GetChar(a_char);

        a_char
    }
}

enum Msg {
    PrintStr(String),
    PrintChar(u16),
    GetChar(u16),
}
