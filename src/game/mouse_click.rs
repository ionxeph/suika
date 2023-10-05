use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::AdditionalMassProperties;

use crate::helpers::get_mouse_pos;
use crate::resources::{NextGenerator, SpawnTime};
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
            commands
                .spawn(create_fruit_bundle(
                    texture_handle,
                    world_position[0],
                    SPAWN_HEIGHT,
                    next_fruit,
                ))
                .insert(AdditionalMassProperties::Mass(MASS));
        }
    }
}
