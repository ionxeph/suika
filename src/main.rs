use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod setup;
use setup::SetupPlugin;

mod game;
use game::GamePlugin;

mod resources;
use resources::{NextGenerator, SpawnTime};

mod constants;
use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.56, 1.0, 0.98)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Suika".to_string(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<SpawnTime>()
        .init_resource::<NextGenerator>()
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(SetupPlugin)
        .add_plugins(GamePlugin)
        .run();
}
