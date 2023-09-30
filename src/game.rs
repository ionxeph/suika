use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::setup::{MainCamera, CONTAINER_HALF_WIDTH};

const CLICK_DELAY: f32 = 0.8;

const SIZES: [f32; 11] = [
    26.0, 40.0, 54.0, 60.0, 77.0, 92.0, 97.0, 129.0, 154.0, 174.0, 204.0,
];
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_click_system);
    }
}

#[derive(Resource)]
pub struct SpawnTime {
    // prevent spawning in quick succession
    timer: Timer,
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
    fn start_new_timer(&mut self) {
        self.timer = Timer::from_seconds(CLICK_DELAY, TimerMode::Once);
    }
}

fn mouse_click_system(
    commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut click_buffer: ResMut<SpawnTime>,
    time: Res<Time>,
) {
    // TODO: add handler for preview that changes its position based on mouse position, should do this regardless of timer
    click_buffer.timer.tick(time.delta());
    if !click_buffer.timer.finished() {
        return;
    }
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
            click_buffer.start_new_timer();
            spawn_yagoo(commands, asset_server, world_position[0]);
        }
    }
}

fn spawn_yagoo(mut commands: Commands, asset_server: Res<AssetServer>, mouse_x: f32) {
    let texture_handle = asset_server.load("trimmed-yagoo.png");
    let num = rand::thread_rng().gen_range(0..5);
    let size = SIZES[num];

    // make sure spawning position is in bounds
    // adding one pixel on either edge to prevent collision against wall on drop
    let pos = match mouse_x {
        x if x < 0.0 => x.max((CONTAINER_HALF_WIDTH * -1.0 + size / 2.0) + 1.0),
        x if x > 0.0 => x.min((CONTAINER_HALF_WIDTH - size / 2.0) - 1.0),
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
        .insert(ColliderMassProperties::Mass(3.0)) // TODO: configure mass according to size
        .insert(Restitution::coefficient(0.3))
        .insert(TransformBundle::from(Transform::from_xyz(pos, 250.0, 0.0)));
}
