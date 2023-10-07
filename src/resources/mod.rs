use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    constants::{ALIVE_MASS_MIN, CLICK_DELAY, KNOWN_TYPES, MASS},
    Fruit,
};

#[derive(Resource, Default)]
pub struct GameAlreadySetUp {
    // prevent game setup systems from running when transitioning from GameOver state to InGame state
    pub is_set_up: bool,
}

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

#[derive(Resource, Default)]
pub struct ScoreTracker {
    pub score: u32,
}

impl ScoreTracker {
    pub fn add_score(&mut self, s: u32) {
        self.score += s;
    }

    pub fn reset(&mut self) {
        self.score = 0;
    }
}

#[derive(Resource)]
pub struct MassSetting {
    pub percentage: f32,
}

impl Default for MassSetting {
    fn default() -> Self {
        Self { percentage: 0.5 }
    }
}

impl MassSetting {
    pub fn get_mass(&self) -> f32 {
        (MASS - ALIVE_MASS_MIN) * self.percentage + ALIVE_MASS_MIN
    }
}

#[derive(Resource)]
pub struct NoiseSetting {
    pub is_on: bool,
}

impl Default for NoiseSetting {
    fn default() -> Self {
        Self { is_on: true }
    }
}

impl NoiseSetting {
    pub fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }
}

#[derive(Resource)]
pub struct NextGenerator {
    pub current_fruit: Fruit,
    pub next_fruit: Fruit,
    pub should_update_previews: bool,
}

impl Default for NextGenerator {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let (cur, next) = (rng.gen_range(0..5), rng.gen_range(0..5));
        let (cur_size, file_name, cur_score) = KNOWN_TYPES[cur];
        let (next_size, next_file_name, next_score) = KNOWN_TYPES[next];
        Self {
            current_fruit: Fruit {
                size: cur_size,
                file_name: String::from(file_name),
                score: cur_score,
            },
            next_fruit: Fruit {
                size: next_size,
                file_name: String::from(next_file_name),
                score: next_score,
            },
            should_update_previews: false,
        }
    }
}

impl NextGenerator {
    pub fn next(&mut self) {
        let mut rng = rand::thread_rng();
        let next = rng.gen_range(0..5);
        self.current_fruit = self.next_fruit.clone();
        let (size, file_name, score) = KNOWN_TYPES[next];
        self.next_fruit = Fruit {
            size,
            file_name: String::from(file_name),
            score,
        };
        self.should_update_previews = true;
    }

    pub fn preview_updated(&mut self) {
        self.should_update_previews = false;
    }
}
