#[derive(Clone, Copy)]
pub struct Register(u16);

impl Register {

    pub fn new() -> Register {
        Register::new_with(0)
    }

    pub fn new_with(n: u16) -> Register {
        Register(n)
    }

    pub fn get_val(&self) -> u16 {
        self.0
    }

    pub fn set_val(&mut self, value: u16) {
        self.0 = value;
    }
}