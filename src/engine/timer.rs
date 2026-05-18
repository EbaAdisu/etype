use std::time::Instant;

pub struct SessionTimer {
    start: Instant,
    pub duration_secs: u64,
}

impl SessionTimer {
    pub fn start(duration_secs: u64) -> Self {
        SessionTimer {
            start: Instant::now(),
            duration_secs,
        }
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    pub fn remaining_secs(&self) -> f64 {
        let elapsed = self.start.elapsed().as_secs_f64();
        (self.duration_secs as f64 - elapsed).max(0.0)
    }

    pub fn is_expired(&self) -> bool {
        self.start.elapsed().as_secs_f64() >= self.duration_secs as f64
    }

    pub fn fraction_remaining(&self) -> f64 {
        self.remaining_secs() / self.duration_secs as f64
    }
}

pub struct KeyTimer {
    last: Instant,
}

impl KeyTimer {
    pub fn new() -> Self {
        KeyTimer {
            last: Instant::now(),
        }
    }

    pub fn elapsed_ms(&self) -> f64 {
        self.last.elapsed().as_secs_f64() * 1000.0
    }

    pub fn reset(&mut self) {
        self.last = Instant::now();
    }
}

impl Default for KeyTimer {
    fn default() -> Self {
        Self::new()
    }
}
