use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod menu;
use menu::MenuPlugin;

mod setup;
use setup::SetupPlugin;

mod game;
use game::GamePlugin;

mod resources;
use resources::{GameAlreadySetUp, NextGenerator, SpawnTime};

mod constants;
use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

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
        .insert_resource(ClearColor(Color::rgb(0.56, 1.0, 0.98)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Suika".to_string(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
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
        .add_plugins(SetupPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .run();
}
