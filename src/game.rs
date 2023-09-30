use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::setup::{MainCamera, CONTAINER_HALF_WIDTH};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_click_system);
    }
}

const SIZES: [f32; 11] = [
    26.0, 40.0, 54.0, 60.0, 77.0, 92.0, 97.0, 129.0, 154.0, 174.0, 204.0,
];

fn mouse_click_system(
    commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(world_position) = q_windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            spawn_yagoo(commands, asset_server, world_position[0]);
        }
    }
}

fn spawn_yagoo(mut commands: Commands, asset_server: Res<AssetServer>, mouse_x: f32) {
    let texture_handle = asset_server.load("trimmed-yagoo.png");
    let num = rand::thread_rng().gen_range(0..11);
    let size = SIZES[num];

    // make sure spawning position is in bounds
    let pos = match mouse_x {
        x if x < 0.0 => x.max(CONTAINER_HALF_WIDTH * -1.0 + size / 2.0),
        x if x > 0.0 => x.min(CONTAINER_HALF_WIDTH - size / 2.0),
        _ => mouse_x,
    };

    commands
        .spawn((
            RigidBody::Dynamic,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0) * size),
                    ..default()
                },
                texture: texture_handle.clone(),
                ..default()
            },
        ))
        .insert(Collider::ball(size / 2.))
        .insert(GravityScale(2.5))
        .insert(ColliderMassProperties::Mass(3.0))
        .insert(Restitution::coefficient(0.3))
        .insert(TransformBundle::from(Transform::from_xyz(pos, 250.0, 0.0)));
}
