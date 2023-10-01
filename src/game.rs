use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::resources::{Fruit, NextGenerator, SpawnTime};
use crate::setup::{MainCamera, Preview, CONTAINER_HALF_WIDTH};

const GRAVITY: f32 = 3.0;
const RESTITUATION: f32 = 0.05;
const MASS: f32 = 5.0;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (mouse_click_system, mouse_move_system, collision_system),
        );
    }
}

fn mouse_move_system(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<(&Preview, &mut Sprite, &mut Handle<Image>, &mut Transform)>,
    asset_server: Res<AssetServer>,
    next_generator: Res<NextGenerator>,
) {
    let mouse_pos = get_mouse_pos(q_windows, camera_q);

    if let Some(world_position) = mouse_pos {
        let pos = pos_x_in_bounds(world_position[0], next_generator.current_fruit.size);
        let (_, mut sprite, mut handle, mut transform) = query.single_mut();
        transform.translation.x = pos;
        sprite.custom_size = Some(Vec2::new(1.0, 1.0) * next_generator.current_fruit.size);
        let texture_handle = asset_server.load(&next_generator.current_fruit.image_file_name);
        *handle = texture_handle;
    }
}

#[allow(clippy::too_many_arguments)]
fn mouse_click_system(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut click_buffer: ResMut<SpawnTime>,
    time: Res<Time>,
    mut next_generator: ResMut<NextGenerator>,
) {
    click_buffer.timer.tick(time.delta());
    if !click_buffer.timer.finished() {
        return;
    }
    let mouse_pos = get_mouse_pos(q_windows, camera_q);

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(world_position) = mouse_pos {
            click_buffer.start_new_timer();
            let next_fruit = next_generator.current_fruit.clone();
            next_generator.next(); // after spawning current, go to next
            let texture_handle = asset_server.load(&next_fruit.image_file_name);
            commands.spawn(create_fruit_bundle(
                texture_handle,
                world_position[0],
                None,
                next_fruit,
            ));
        }
    }
}

fn collision_system(
    mut collisions: EventReader<CollisionEvent>,
    asset_server: Res<AssetServer>,
    fruits: Query<(&Fruit, &mut Transform)>,
    mut commands: Commands,
) {
    for collision in collisions.iter() {
        if let CollisionEvent::Started(collider_a, collider_b, _) = collision {
            if let Ok([(fruit_a, transform_a), (fruit_b, transform_b)]) =
                fruits.get_many([*collider_a, *collider_b])
            {
                if fruit_a.size == fruit_b.size {
                    let new_x = (transform_a.translation.x + transform_b.translation.x) / 2.0;
                    let new_y = (transform_a.translation.y + transform_b.translation.y) / 2.0;
                    // Fruit.merged_size returns None if two largest fruits collide
                    // in this case, both are despawned, and no new fruits created
                    if let Some(fruit) = fruit_a.merge() {
                        let texture_handle = asset_server.load(&fruit.image_file_name);

                        commands.spawn(create_fruit_bundle(
                            texture_handle,
                            new_x,
                            Some(new_y),
                            fruit,
                        ));
                    }

                    commands.entity(*collider_a).despawn();
                    commands.entity(*collider_b).despawn();
                }
            }
        }
    }
}

fn get_mouse_pos(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();
    q_windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
}

fn create_fruit_bundle(
    texture_handle: Handle<Image>,
    pos_x: f32,
    pos_y: Option<f32>,
    fruit: Fruit,
) -> (
    Fruit,
    RigidBody,
    SpriteBundle,
    Collider,
    GravityScale,
    ColliderMassProperties,
    Restitution,
    ActiveEvents,
) {
    // make sure spawning position is in bounds
    // adding one pixel on either edge to prevent collision against wall on drop
    let size = fruit.size;
    let pos_x_in_bounds = pos_x_in_bounds(pos_x, size);
    let pos_y_in_bounds = pos_y.unwrap_or(250.0); // TODO: make sure this is in bounds
    (
        fruit,
        RigidBody::Dynamic,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0) * size),
                ..default()
            },
            texture: texture_handle,
            transform: Transform::from_xyz(pos_x_in_bounds, pos_y_in_bounds, 0.0),
            ..default()
        },
        Collider::ball(size / 2.0),
        GravityScale(GRAVITY),
        ColliderMassProperties::Mass(MASS), // TODO: figure out if this requires changing
        Restitution::coefficient(RESTITUATION),
        ActiveEvents::COLLISION_EVENTS,
    )
}

fn pos_x_in_bounds(raw_x: f32, sprite_size: f32) -> f32 {
    match raw_x {
        x if x < 0.0 => x.max((CONTAINER_HALF_WIDTH * -1.0 + sprite_size / 2.0) + 1.0),
        x if x > 0.0 => x.min((CONTAINER_HALF_WIDTH - sprite_size / 2.0) - 1.0),
        _ => raw_x,
    }
}
