use bevy::audio::Volume;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::AdditionalMassProperties;
use rand::prelude::*;

use crate::helpers::{
    get_mouse_pos, mouse_pos_in_noise_toggle, mouse_pos_in_restart, mouse_pos_in_slider,
};
use crate::resources::{NextGenerator, NoiseSetting, SpawnTime};
use crate::setup::MainCamera;

use crate::constants::{MASS, SPAWN_HEIGHT};

use super::create_fruit_bundle;

#[allow(clippy::too_many_arguments)]
pub fn mouse_click(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut click_buffer: ResMut<SpawnTime>,
    time: Res<Time>,
    mut next_generator: ResMut<NextGenerator>,
    noise_setting: Res<NoiseSetting>,
) {
    click_buffer.timer.tick(time.delta());
    if !click_buffer.timer.finished() {
        return;
    }
    let mouse_pos = get_mouse_pos(q_windows, camera_q);

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(world_position) = mouse_pos {
            if !mouse_pos_in_slider(world_position)
                && !mouse_pos_in_noise_toggle(world_position)
                && !mouse_pos_in_restart(world_position)
            {
                click_buffer.start_new_timer();
                let next_fruit = next_generator.current_fruit.clone();
                let file_name = &next_fruit.file_name;
                next_generator.next(); // after spawning current, go to next
                let texture_handle = asset_server.load(format!("{}.png", file_name));
                let mut rng = rand::thread_rng();
                let mouse_x = world_position[0] + rng.gen_range(-1.0..1.0);
                let mut spawned_fruit = commands.spawn((
                    create_fruit_bundle(texture_handle, mouse_x, SPAWN_HEIGHT, next_fruit.clone()),
                    AdditionalMassProperties::Mass(MASS),
                ));

                if noise_setting.is_on {
                    spawned_fruit.insert(AudioBundle {
                        source: asset_server.load(format!("audio/{}.ogg", file_name)),
                        settings: PlaybackSettings {
                            volume: Volume::new_absolute(0.5),
                            ..default()
                        },
                    });
                }
            }
        }
    }
}
