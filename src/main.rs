use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod menu;
use menu::MenuPlugin;

mod setup;
use setup::SetupPlugin;

mod game;
use game::GamePlugin;

mod resources;
use resources::{
    GameAlreadySetUp, MassSetting, NextGenerator, NoiseSetting, ScoreTracker, SpawnTime,
};

mod constants;
use constants::{BG_COLOR, KNOWN_TYPES, SCREEN_HEIGHT, SCREEN_WIDTH};

mod helpers;

fn main() {
    App::new()
        .add_state::<AppState>()
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Suika".to_string(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),
        ))
        .init_resource::<SpawnTime>()
        .init_resource::<NextGenerator>()
        .init_resource::<GameAlreadySetUp>()
        .init_resource::<ScoreTracker>()
        .init_resource::<MassSetting>()
        .init_resource::<NoiseSetting>()
        .add_plugins(SetupPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    StartMenu,
    InGame,
    GameOverMenu,
}

#[derive(Component)]
pub struct Fruit {
    pub size: f32,
    pub file_name: String,
    pub score: u32,
}

impl Fruit {
    pub fn merge(&self) -> Option<Fruit> {
        if self.size == KNOWN_TYPES[10].0 {
            return None;
        }
        let (size, file_name, score) =
            KNOWN_TYPES[KNOWN_TYPES.iter().position(|s| s.0 == self.size).unwrap() + 1];
        Some(Fruit {
            size,
            file_name: String::from(file_name),
            score,
        })
    }
}

impl Clone for Fruit {
    fn clone(&self) -> Fruit {
        Fruit {
            size: self.size,
            file_name: self.file_name.clone(),
            score: self.score,
        }
    }
}
