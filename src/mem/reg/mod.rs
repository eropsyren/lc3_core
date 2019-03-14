#[derive(Clone, Copy)]
pub struct Register(u16);

impl Register {
    pub fn new() -> Register {
        Register::with(0)
    }

    pub fn with(n: u16) -> Register {
        Register(n)
    }

    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, value: u16) {
        self.0 = value;
    }
}

#[cfg(test)]
mod tests {

    use super::Register;

    #[test]
    fn test_new() {
        let r = Register::new();

        assert_eq!(r.0, 0);
    }

    #[test]
    fn test_with() {
        let r = Register::with(1);

        assert_eq!(r.0, 1);
    }

    #[test]
    fn test_get() {
        let r = Register::with(1);

        assert_eq!(r.get(), 1);
    }

    #[test]
    fn test_set() {
        let mut r = Register::new();
        r.set(1);

        assert_eq!(r.0, 1);
    }
}
