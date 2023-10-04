use std::time::Duration;

use bevy::time::common_conditions::on_fixed_timer;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::helpers::get_mouse_pos;
use crate::resources::{Fruit, NextGenerator, ScoreTracker, SpawnTime};
use crate::setup::{MainCamera, NextPreview, Preview, Score};
use crate::AppState;

use crate::constants::{
    CONTAINER_THICKNESS, CONTAINER_WIDTH, GAME_OVER_HEIGHT, GRAVITY, MASS, MAX_SPEED,
    MAX_X_VELOCITY_BEFORE_CLAMP, MAX_Y_VELOCITY_BEFORE_CLAMP, RESTITUATION, SPAWN_HEIGHT,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                mouse_click_system,
                update_preview_system,
                collision_system,
                clamp_upward_velocity,
                check_game_over,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            remove_used_fruits
                .run_if(on_fixed_timer(Duration::from_secs(2)))
                .run_if(in_state(AppState::InGame)),
        );
    }
}

// tracks fruits that should be considered for game over conditions
#[derive(Component)]
pub struct Alive;

#[derive(Component)]
pub struct MarkForDelete;

fn check_game_over(
    positions: Query<&Transform, With<Alive>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for position in positions.iter() {
        if position.translation.y > GAME_OVER_HEIGHT {
            next_state.set(AppState::GameOverMenu);
        }
        if position.translation.x > CONTAINER_WIDTH / 2.0 + CONTAINER_THICKNESS
            || position.translation.x < -CONTAINER_WIDTH / 2.0 - CONTAINER_THICKNESS
        {
            next_state.set(AppState::GameOverMenu);
        }
    }
}

fn remove_used_fruits(
    fruits_marked_for_delete: Query<Entity, With<MarkForDelete>>,
    mut commands: Commands,
) {
    for fruit in fruits_marked_for_delete.iter() {
        commands.entity(fruit).despawn();
    }
}

fn clamp_upward_velocity(mut velocities: Query<&mut Velocity>) {
    for mut vel in velocities.iter_mut() {
        if vel.linvel.y > MAX_Y_VELOCITY_BEFORE_CLAMP {
            vel.linvel = vel.linvel.clamp_length_max(MAX_SPEED);
        }
        if vel.linvel.x > MAX_X_VELOCITY_BEFORE_CLAMP {
            vel.linvel = vel.linvel.clamp_length_max(MAX_SPEED);
        }
    }
}

fn update_preview_system(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut preview: Query<(&Preview, &mut Sprite, &mut Handle<Image>, &mut Transform)>,
    mut next_preview: Query<(&NextPreview, &mut Sprite, &mut Handle<Image>), Without<Preview>>,
    asset_server: Res<AssetServer>,
    mut next_generator: ResMut<NextGenerator>,
) {
    let mouse_pos = get_mouse_pos(q_windows, camera_q);

    if let Some(world_position) = mouse_pos {
        let pos = pos_x_in_bounds(world_position[0], next_generator.current_fruit.size);
        // update current preview
        let (_, mut sprite, mut handle, mut transform) = preview.single_mut();
        transform.translation.x = pos;

        // if preview images and sizes need to be updated
        if next_generator.should_update_previews {
            sprite.custom_size = Some(Vec2::new(1.0, 1.0) * next_generator.current_fruit.size);
            let texture_handle = asset_server.load(&next_generator.current_fruit.image_file_name);
            *handle = texture_handle;

            // update next preview
            let (_, mut next_sprite, mut next_handle) = next_preview.single_mut();
            next_sprite.custom_size = Some(Vec2::new(1.0, 1.0) * next_generator.next_fruit.size);
            let next_texture_handle = asset_server.load(&next_generator.next_fruit.image_file_name);
            *next_handle = next_texture_handle;
            next_generator.preview_updated();
        }
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
                SPAWN_HEIGHT,
                next_fruit,
            ));
        }
    }
}

fn collision_system(
    mut collisions: EventReader<CollisionEvent>,
    asset_server: Res<AssetServer>,
    mut score_tracker: ResMut<ScoreTracker>,
    fruits: Query<(&Fruit, &mut Transform)>,
    mut score_query: Query<&mut Text, With<Score>>,
    not_alive_fruits: Query<&Fruit, Without<Alive>>,
    mut commands: Commands,
) {
    for collision in collisions.iter() {
        if let CollisionEvent::Started(collider_a, collider_b, _) = collision {
            if not_alive_fruits.get(*collider_a).is_ok() {
                let mut entity = commands.entity(*collider_a);
                entity.insert(Alive);
                entity.remove::<ColliderMassProperties>();
                entity.insert(ColliderMassProperties::Mass(0.001));
            }

            if not_alive_fruits.get(*collider_b).is_ok() {
                let mut entity = commands.entity(*collider_b);
                entity.insert(Alive);
                entity.remove::<ColliderMassProperties>();
                entity.insert(ColliderMassProperties::Mass(0.001));
            }

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
                        score_tracker.add_score(fruit.score);
                        let mut score = score_query.single_mut();
                        score.sections[0].value = score_tracker.score.to_string();
                        commands.spawn(create_fruit_bundle(texture_handle, new_x, new_y, fruit));
                    }
                    commands
                        .entity(*collider_a)
                        .remove::<(RigidBody, SpriteBundle, Collider)>()
                        .insert(MarkForDelete);
                    commands
                        .entity(*collider_b)
                        .remove::<(RigidBody, SpriteBundle, Collider)>()
                        .insert(MarkForDelete);
                }
            }
        }
    }
}

fn create_fruit_bundle(
    texture_handle: Handle<Image>,
    pos_x: f32,
    pos_y: f32,
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
    Velocity,
) {
    // make sure spawning position is in bounds
    // adding one pixel on either edge to prevent collision against wall on drop
    let size = fruit.size;
    let pos_x_in_bounds = pos_x_in_bounds(pos_x, size);
    (
        fruit,
        RigidBody::Dynamic,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0) * size),
                ..default()
            },
            texture: texture_handle,
            transform: Transform::from_xyz(pos_x_in_bounds, pos_y, 0.0),
            ..default()
        },
        Collider::ball(size / 2.0),
        GravityScale(GRAVITY),
        ColliderMassProperties::Mass(MASS),
        Restitution::coefficient(RESTITUATION),
        ActiveEvents::COLLISION_EVENTS,
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        },
    )
}

fn pos_x_in_bounds(raw_x: f32, sprite_size: f32) -> f32 {
    match raw_x {
        x if x < 0.0 => x.max((-CONTAINER_WIDTH / 2.0 + sprite_size / 2.0) + 1.0),
        x if x > 0.0 => x.min((CONTAINER_WIDTH / 2.0 - sprite_size / 2.0) - 1.0),
        _ => raw_x,
    }
}
