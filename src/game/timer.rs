use std::time::{Duration, Instant};

pub struct Timer {
    start_time: Instant,
    elapsed: Duration,
    stopped: bool,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_time: Instant::now(),
            elapsed: Duration::new(0, 0),
            stopped: false,
        }
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
        self.elapsed = Duration::new(0, 0);
        self.stopped = true;
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
        self.elapsed = Duration::new(0, 0);
        self.stopped = false;
    }

    pub fn stop(&mut self) {
        self.elapsed = self.start_time.elapsed();
        self.stopped = true;
    }
    pub fn elapsed(&self) -> Duration {
        if self.stopped {
            self.elapsed
        } else {
            self.start_time.elapsed()
        }
    }
}
