use bevy::prelude::*;
use rand::Rng;

pub const SCREEN_WIDTH: f32 = 300.0;
pub const SCREEN_HEIGHT: f32 = 400.0;

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
        .add_systems(Startup, (setup_contributor_selection, setup))
        .add_systems(Update, (velocity_system, move_system, collision_system))
        .run();
}

#[derive(Component)]
struct Fruit;

#[derive(Component)]
struct Velocity {
    translation: Vec3,
    rotation: f32,
}

const GRAVITY: f32 = 9.821 * 100.0;
const SPRITE_SIZE: f32 = 75.0;

fn setup_contributor_selection(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("yagoo.png");
    let mut rng = rand::thread_rng();

    for _ in 0..=3 {
        let pos = (rng.gen_range(-400.0..400.0), rng.gen_range(0.0..400.0));
        let dir = rng.gen_range(-1.0..1.0);
        let velocity = Vec3::new(dir * 500.0, 0.0, 0.0);

        // some sprites should be flipped
        let flipped = rng.gen_bool(0.5);

        let transform = Transform::from_xyz(pos.0, pos.1, 0.0);

        commands.spawn((
            Fruit,
            Velocity {
                translation: velocity,
                rotation: -dir * 5.0,
            },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0) * SPRITE_SIZE),
                    flip_x: flipped,
                    ..default()
                },
                texture: texture_handle.clone(),
                transform,
                ..default()
            },
        ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Applies gravity to all entities with velocity
fn velocity_system(time: Res<Time>, mut velocity_query: Query<&mut Velocity>) {
    let delta = time.delta_seconds();

    for mut velocity in &mut velocity_query {
        velocity.translation.y -= GRAVITY * delta;
    }
}

/// Checks for collisions of contributor-birds.
///
/// On collision with left-or-right wall it resets the horizontal
/// velocity. On collision with the ground it applies an upwards
/// force.
fn collision_system(
    windows: Query<&Window>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Fruit>>,
) {
    let window = windows.single();

    let ceiling = window.height() / 2.;
    let ground = -window.height() / 2.;

    let wall_left = -window.width() / 2.;
    let wall_right = window.width() / 2.;

    // The maximum height the birbs should try to reach is one birb below the top of the window.
    let max_bounce_height = (window.height() - SPRITE_SIZE * 2.0).max(0.0);

    let mut rng = rand::thread_rng();

    for (mut velocity, mut transform) in &mut query {
        let left = transform.translation.x - SPRITE_SIZE / 2.0;
        let right = transform.translation.x + SPRITE_SIZE / 2.0;
        let top = transform.translation.y + SPRITE_SIZE / 2.0;
        let bottom = transform.translation.y - SPRITE_SIZE / 2.0;

        // clamp the translation to not go out of the bounds
        if bottom < ground {
            transform.translation.y = ground + SPRITE_SIZE / 2.0;

            // How high this birb will bounce.
            let bounce_height = rng.gen_range((max_bounce_height * 0.4)..=max_bounce_height);

            // Apply the velocity that would bounce the birb up to bounce_height.
            velocity.translation.y = (bounce_height * GRAVITY * 2.).sqrt();
        }
        if top > ceiling {
            transform.translation.y = ceiling - SPRITE_SIZE / 2.0;
            velocity.translation.y *= -1.0;
        }
        // on side walls flip the horizontal velocity
        if left < wall_left {
            transform.translation.x = wall_left + SPRITE_SIZE / 2.0;
            velocity.translation.x *= -1.0;
            velocity.rotation *= -1.0;
        }
        if right > wall_right {
            transform.translation.x = wall_right - SPRITE_SIZE / 2.0;
            velocity.translation.x *= -1.0;
            velocity.rotation *= -1.0;
        }
    }
}

/// Apply velocity to positions and rotations.
fn move_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    let delta = time.delta_seconds();

    for (velocity, mut transform) in &mut query {
        transform.translation += delta * velocity.translation;
        transform.rotate_z(velocity.rotation * delta);
    }
}
