use crate::controller::Controller;
use crate::mem::Memory;

pub fn halt(mem: &mut Memory, controller: &mut Controller) {
    controller.stop();
}

#[cfg(test)]
mod tests {
    use super::halt;
    use crate::controller::Controller;
    use crate::mem::Memory;

    #[test]
    fn test_halt() {
        let mut mem = Memory::new();
        let mut controller = Controller::new();

        halt(&mut mem, &mut controller);

        assert_eq!(false, controller.is_running());
    }
}
