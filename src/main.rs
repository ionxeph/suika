use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod setup;
use setup::SetupPlugin;

mod game;
use game::GamePlugin;

const SCREEN_WIDTH: f32 = 400.0;
const SCREEN_HEIGHT: f32 = 800.0;

fn main() {
    App::new()
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
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(SetupPlugin)
        .add_plugins(GamePlugin)
        .run();
}
