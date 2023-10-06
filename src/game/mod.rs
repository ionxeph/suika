use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_fixed_timer;
use bevy_rapier2d::prelude::*;

use crate::{AppState, Fruit};

use crate::constants::{CONTAINER_WIDTH, GRAVITY, RESTITUATION};

mod mouse_click;
use mouse_click::mouse_click;

mod update_preview;
use update_preview::update_preview;

mod collision;
use collision::{collision, remove_used_fruits};

mod game_over;
use game_over::check_game_over;

mod physics_manipulations;
use physics_manipulations::{clamp_upward_velocity, manipulate_mass, mark_fruits_as_alive};

use self::collision::merge_fruits;
use self::physics_manipulations::change_manipulated_mass_on_slide;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                mouse_click,
                update_preview,
                collision,
                merge_fruits,
                clamp_upward_velocity,
                mark_fruits_as_alive,
                manipulate_mass,
                change_manipulated_mass_on_slide,
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
pub struct TimeSinceSpawn {
    pub timer: Timer,
}

pub fn create_fruit_bundle(
    texture_handle: Handle<Image>,
    pos_x: f32,
    pos_y: f32,
    fruit: Fruit,
) -> (
    Fruit,
    TimeSinceSpawn,
    RigidBody,
    SpriteBundle,
    Collider,
    GravityScale,
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
        TimeSinceSpawn {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        },
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
        Restitution::coefficient(RESTITUATION),
        ActiveEvents::COLLISION_EVENTS,
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        },
    )
}

pub fn pos_x_in_bounds(raw_x: f32, sprite_size: f32) -> f32 {
    match raw_x {
        x if x < 0.0 => x.max((-CONTAINER_WIDTH / 2.0 + sprite_size / 2.0) + 1.0),
        x if x > 0.0 => x.min((CONTAINER_WIDTH / 2.0 - sprite_size / 2.0) - 1.0),
        _ => raw_x,
    }
}
