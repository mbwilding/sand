use std::time::{Duration, Instant};

pub struct FpsCounter {
    pub fps: f64,
    last_instant: Instant,
    frame_count: usize,
}

impl FpsCounter {
    pub fn new() -> Self {
        FpsCounter {
            fps: 0.0,
            last_instant: Instant::now(),
            frame_count: 0,
        }
    }

    pub fn tick(&mut self) {
        self.frame_count += 1;
        let now = Instant::now();
        let duration = now.duration_since(self.last_instant);

        if duration >= Duration::from_millis(100) {
            self.fps = self.frame_count as f64 / duration.as_secs_f64();
            self.frame_count = 0;
            self.last_instant = now;
        }
    }
}
