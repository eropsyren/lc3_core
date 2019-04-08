pub struct Controller {
    running: bool,
}

impl Controller {
    pub fn new() -> Controller {
        Controller { running: false }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}

#[cfg(test)]
mod tests {

    use super::Controller;

    #[test]
    fn test_controller_new() {
        let ctrl = Controller::new();

        assert_eq!(ctrl.running, false);
    }

    #[test]
    fn test_controller_is_running() {
        let mut ctrl0 = Controller::new();
        ctrl0.start();

        let mut ctrl1 = Controller::new();
        ctrl1.stop();

        assert_eq!(ctrl0.is_running(), true);
        assert_eq!(ctrl1.is_running(), false);
    }

    #[test]
    fn test_controller_start() {
        let mut ctrl = Controller::new();

        ctrl.start();

        assert_eq!(ctrl.running, true);
    }

    #[test]
    fn test_controller_stop() {
        let mut ctrl = Controller::new();

        ctrl.stop();

        assert_eq!(ctrl.running, false);
    }
}
