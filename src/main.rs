use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

pub const SCREEN_WIDTH: f32 = 400.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

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
        .add_systems(Startup, (setup_container, setup_camera))
        .add_systems(Update, mouse_click_system)
        .run();
}

const SPRITE_SIZE: f32 = 75.0;

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn setup_container(mut commands: Commands) {
    /* Create the container. */
    commands
        .spawn(Collider::cuboid(150.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -275.0, 0.0)));
    commands
        .spawn(Collider::cuboid(10.0, 275.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            160.0, -10.0, 0.0,
        )));
    commands
        .spawn(Collider::cuboid(10.0, 275.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            -160.0, -10.0, 0.0,
        )));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

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

fn spawn_yagoo(mut commands: Commands, asset_server: Res<AssetServer>, x: f32) {
    let texture_handle = asset_server.load("trimmed-yagoo.png");
    commands
        .spawn((
            RigidBody::Dynamic,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0) * SPRITE_SIZE),
                    ..default()
                },
                texture: texture_handle.clone(),
                ..default()
            },
        ))
        .insert(Collider::ball(SPRITE_SIZE / 2.))
        .insert(GravityScale(2.5))
        .insert(ColliderMassProperties::Mass(3.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(x, 250.0, 0.0)));
}
