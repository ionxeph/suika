use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod menu;
use menu::MenuPlugin;

mod setup;
use setup::SetupPlugin;

mod game;
use game::GamePlugin;

mod resources;
use resources::{GameAlreadySetUp, NextGenerator, ScoreTracker, SpawnTime};

mod constants;
use constants::{BG_COLOR, SCREEN_HEIGHT, SCREEN_WIDTH};

mod helpers;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    StartMenu,
    InGame,
    GameOverMenu,
}

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
        .add_plugins(SetupPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .run();
}
