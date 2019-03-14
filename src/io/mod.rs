pub trait IODevice {
    fn print_str(&self, str: &str);

    fn print_char(&self, c: u16);

    fn get_char(&self) -> u16;
}
