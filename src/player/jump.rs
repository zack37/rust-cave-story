use time::Duration;

const JUMP_TIME: i64 = 275; // ms

pub struct Jump {
    time_remaining: Duration,
    active: bool,
}

impl Jump {
    pub fn new() -> Jump {
        Jump {
            time_remaining: Duration::zero(),
            active: false,
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn reset(&mut self) {
        self.time_remaining = Duration::milliseconds(JUMP_TIME);
        self.reactivate();
    }

    pub fn reactivate(&mut self) {
        self.active = self.time_remaining > Duration::zero();
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn update(&mut self, elapsed_time: Duration) {
        if self.active {
            self.time_remaining = self.time_remaining - elapsed_time;
            if self.time_remaining <= Duration::zero() {
                self.active = false;
            }
        }
    }
}
