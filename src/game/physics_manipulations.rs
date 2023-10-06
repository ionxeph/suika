use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::{MAX_SPEED, MAX_X_VELOCITY_BEFORE_CLAMP, MAX_Y_VELOCITY_BEFORE_CLAMP},
    resources::MassSetting,
    Fruit,
};

use super::{Alive, TimeSinceSpawn};

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

#[allow(clippy::type_complexity)]
pub fn mark_fruits_as_alive(
    mut unalive_fruits: Query<(Entity, &mut TimeSinceSpawn), (With<Fruit>, Without<Alive>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut time_since_spawn) in unalive_fruits.iter_mut() {
        time_since_spawn.timer.tick(time.delta());
        if !time_since_spawn.timer.finished() {
            return;
        }

        commands.entity(entity).insert(Alive);
        commands.entity(entity).remove::<TimeSinceSpawn>();
    }
}

#[derive(Component)]
pub struct MassAltered;

#[allow(clippy::type_complexity)]
pub fn manipulate_mass(
    mut alive_fruits: Query<
        (&Velocity, &mut AdditionalMassProperties, Entity),
        (With<Alive>, Without<MassAltered>),
    >,
    mut commands: Commands,
    mass_setting: Res<MassSetting>,
) {
    for (vel, mut mprops, entity) in alive_fruits.iter_mut() {
        if vel.linvel.abs().y < 1.0 && vel.linvel.abs().x < 1.0 {
            *mprops = AdditionalMassProperties::Mass(mass_setting.get_mass());
            commands.entity(entity).insert(MassAltered);
        }
    }
}

pub fn change_manipulated_mass_on_slide(
    mut altered_masses: Query<&mut AdditionalMassProperties, With<MassAltered>>,
    mass_setting: Res<MassSetting>,
) {
    if mass_setting.is_changed() {
        for mut mprops in altered_masses.iter_mut() {
            *mprops = AdditionalMassProperties::Mass(mass_setting.get_mass());
        }
    }
}
