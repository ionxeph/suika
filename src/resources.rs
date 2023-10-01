use bevy::prelude::*;
use rand::prelude::*;

const CLICK_DELAY: f32 = 0.8;

const SIZES: [f32; 11] = [
    26.0, 40.0, 54.0, 60.0, 77.0, 92.0, 97.0, 129.0, 154.0, 174.0, 204.0,
];

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

#[derive(Resource)]
pub struct NextGenerator {
    pub current_fruit: Fruit,
    pub next_fruit: Fruit,
}

impl Default for NextGenerator {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let (cur, next) = (rng.gen_range(0..5), rng.gen_range(0..5));
        Self {
            current_fruit: Fruit { size: SIZES[cur] },
            next_fruit: Fruit { size: SIZES[next] },
        }
    }
}

impl NextGenerator {
    pub fn next(&mut self) {
        let mut rng = rand::thread_rng();
        let next = rng.gen_range(0..5);
        self.current_fruit = Fruit {
            size: self.next_fruit.size,
        };
        self.next_fruit = Fruit { size: SIZES[next] }
    }
}

#[derive(Component)]
pub struct Fruit {
    // TODO: add sprite field when adding more than just yagoo
    pub size: f32,
}
