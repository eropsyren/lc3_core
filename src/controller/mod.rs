pub struct Controller {
    running: bool,
}

impl Controller {
    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self)  {
        self.running = false;
    }
}