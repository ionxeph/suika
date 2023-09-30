use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const CONTAINER_HALF_WIDTH: f32 = 150.0;
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_container, setup_camera));
    }
}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

fn setup_container(mut commands: Commands) {
    /* Create the container. */
    commands
        .spawn(Collider::cuboid(CONTAINER_HALF_WIDTH, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -375.0, 0.0)));
    commands
        .spawn(Collider::cuboid(10.0, 275.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            CONTAINER_HALF_WIDTH + 10.0,
            -110.0,
            0.0,
        )));
    commands
        .spawn(Collider::cuboid(10.0, 275.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            CONTAINER_HALF_WIDTH * -1.0 - 10.0,
            -110.0,
            0.0,
        )));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
