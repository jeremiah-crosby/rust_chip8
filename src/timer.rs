use std::time::{Duration, Instant};

pub struct Timer {
    value: u8,
    interval_millis: u32,
    instant: Instant,
}

impl Timer {
    pub fn new(clock_speed_hz: u32) -> Self {
        Timer {
            value: 0,
            interval_millis: 1000 / clock_speed_hz,
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
        if elapsed > Duration::from_millis(self.interval_millis.into()) {
            self.value -= 1;
            self.instant = Instant::now();
        }
    }
}
