use bevy::prelude::*;

const CLICK_DELAY: f32 = 0.8;

#[derive(Resource)]
pub struct SpawnTime {
    // prevent spawning in quick succession
    pub timer: Timer,
}

impl Default for SpawnTime {
    fn default() -> Self {
        Self {
            // default to 0 seconds as first click doesn't need buffer
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

impl SpawnTime {
    pub fn start_new_timer(&mut self) {
        self.timer = Timer::from_seconds(CLICK_DELAY, TimerMode::Once);
    }
}
