use std::time::{Duration, Instant};

pub struct Timer {
    value: u8,
    instant: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            value: 0,
            instant: Instant::now(),
        }
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
        self.instant = Instant::now();
    }

    pub fn get_value(&mut self) -> u8 {
        self.value
    }

    pub fn tick(&mut self) {
        if self.value == 0 {
            return;
        }

        let elapsed = self.instant.elapsed();
        if elapsed > Duration::from_millis(16) {
            self.value -= 1;
            self.instant = Instant::now();
        }
    }
}
