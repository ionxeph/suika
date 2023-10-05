use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::{MAX_SPEED, MAX_X_VELOCITY_BEFORE_CLAMP, MAX_Y_VELOCITY_BEFORE_CLAMP};
pub fn clamp_upward_velocity(mut velocities: Query<&mut Velocity>) {
    for mut vel in velocities.iter_mut() {
        if vel.linvel.y > MAX_Y_VELOCITY_BEFORE_CLAMP {
            vel.linvel = vel.linvel.clamp_length_max(MAX_SPEED);
        }
        if vel.linvel.x > MAX_X_VELOCITY_BEFORE_CLAMP {
            vel.linvel = vel.linvel.clamp_length_max(MAX_SPEED);
        }
    }
}
