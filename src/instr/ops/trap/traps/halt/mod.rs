use crate::controller::Controller;

pub fn halt(controller: &mut Controller) {
    controller.stop();
}

#[cfg(test)]
mod tests {
    use super::halt;
    use crate::controller::Controller;

    #[test]
    fn test_halt() {
        let mut controller = Controller::new();

        halt(&mut controller);

        assert_eq!(false, controller.is_running());
    }
}
