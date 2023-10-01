use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::{
        CONTAINER_BASE_OFFSET, CONTAINER_HEIGHT, CONTAINER_THICKNESS, CONTAINER_WIDTH,
        SCREEN_HEIGHT,
    },
    resources::NextGenerator,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_container, setup_camera, setup_preview));
    }
}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Preview;

fn setup_container(mut commands: Commands) {
    let container_base = -SCREEN_HEIGHT / 2.0 + CONTAINER_BASE_OFFSET;
    /* Create the container. */
    commands.spawn((
        Collider::cuboid(CONTAINER_WIDTH / 2.0, CONTAINER_THICKNESS / 2.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CONTAINER_WIDTH, CONTAINER_THICKNESS)),
                color: Color::rgb(0.0, 0.0, 0.55),
                ..default()
            },
            transform: Transform::from_xyz(0.0, container_base, 0.0),
            ..default()
        },
    ));

    let wall_height = CONTAINER_HEIGHT + CONTAINER_THICKNESS;
    let wall_base = container_base + CONTAINER_HEIGHT / 2.0;
    commands.spawn((
        Collider::cuboid(CONTAINER_THICKNESS / 2.0, wall_height / 2.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CONTAINER_THICKNESS, wall_height)),
                color: Color::rgb(0.0, 0.0, 0.55),
                ..default()
            },
            transform: Transform::from_xyz(
                (CONTAINER_WIDTH + CONTAINER_THICKNESS) / 2.0,
                wall_base,
                0.0,
            ),
            ..default()
        },
    ));
    commands.spawn((
        Collider::cuboid(CONTAINER_THICKNESS / 2.0, wall_height / 2.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CONTAINER_THICKNESS, wall_height)),
                color: Color::rgb(0.0, 0.0, 0.55),
                ..default()
            },
            transform: Transform::from_xyz(
                -(CONTAINER_WIDTH + CONTAINER_THICKNESS) / 2.0,
                wall_base,
                0.0,
            ),
            ..default()
        },
    ));
}

fn setup_preview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    next_generator: Res<NextGenerator>,
) {
    let file_name = &next_generator.current_fruit.image_file_name;
    let texture_handle = asset_server.load(file_name);
    commands.spawn((
        Preview,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0) * next_generator.current_fruit.size),
                ..default()
            },
            texture: texture_handle,
            transform: Transform::from_xyz(0.0, 250.0, 0.0),
            ..default()
        },
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
