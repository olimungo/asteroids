use crate::utils::millis;

pub struct Interval {
    interval: f64,
    time_reference: f64,
    time_paused: f64,
    pub not_yet_elapsed: bool,
}

impl Interval {
    pub fn new() -> Interval {
        Interval {
            interval: 0.0,
            time_reference: 0.0,
            time_paused: 0.0,
            not_yet_elapsed: true,
        }
    }

    pub fn set_interval_frequency(&mut self, interval: u32) {
        self.interval = interval as f64;
        self.time_reference = millis();
        self.not_yet_elapsed = true;
    }

    pub fn become_elapsed(&mut self) -> bool {
        if self.not_yet_elapsed && self.time_paused == 0.0 && self.interval > 0.0 {
            let now = millis();

            if self.time_reference + self.interval < now {
                self.not_yet_elapsed = false;

                return true;
            }
        }

        false
    }

    pub fn pause(&mut self) {
        if self.not_yet_elapsed {
            self.time_paused = millis();
        }
    }

    pub fn unpause(&mut self) {
        if self.not_yet_elapsed {
            let now = millis();
            let time_left = self.time_reference + self.interval - self.time_paused;
            self.time_reference = now + time_left - self.interval;
            self.time_paused = 0.0;
        }
    }
}
