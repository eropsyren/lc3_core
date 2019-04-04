mod mock;

pub use mock::MockIODevice;
pub use mock::Msg;
pub use mock::Val;

pub trait IODevice {
    fn print_str(&mut self, str: &str);

    fn print_char(&mut self, c: u16);

    fn get_char(&mut self) -> u16;
}
