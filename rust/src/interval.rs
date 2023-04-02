use crate::utils::millis;

pub struct Interval {
    pub interval: f64,
    time_reference: f64,
    time_paused: f64,
}

impl Interval {
    pub fn new() -> Interval {
        Interval {
            interval: 0.0,
            time_reference: 0.0,
            time_paused: 0.0,
        }
    }

    pub fn set(&mut self, interval: u32) {
        self.interval = interval as f64;
        self.time_paused = 0.0;
        self.time_reference = millis();
    }

    pub fn is_ellapsed(&mut self) -> bool {
        if self.time_paused == 0.0 && self.interval > 0.0 {
            let now = millis();

            if self.time_reference + self.interval < now {
                self.time_reference = now;
                return true;
            }
        }

        false
    }

    pub fn cancel(&mut self) {
        self.interval = 0.0;
    }

    pub fn pause(&mut self) {
        self.time_paused = millis();
    }

    pub fn unpause(&mut self) {
        let now = millis();
        let time_left = self.time_reference + self.interval - self.time_paused;
        self.time_reference = now + time_left - self.interval;
        self.time_paused = 0.0;
    }
}
