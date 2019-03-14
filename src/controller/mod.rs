pub struct Controller {
    running: bool,
}

impl Controller {

    pub fn new() -> Controller {
        Controller {
            running: true,
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running()
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self)  {
        self.running = false;
    }
}