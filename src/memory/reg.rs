pub struct Register(u16);

impl Register {

    pub fn get_val(&self) -> u16 {
        self.0
    }

    pub fn set_val(&mut self, value: u16) {
        self.0 = value;
    }
}