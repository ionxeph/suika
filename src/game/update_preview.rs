use crate::helpers::get_mouse_pos;
use crate::resources::NextGenerator;
use crate::setup::{MainCamera, NextPreview, Preview};
use bevy::{prelude::*, window::PrimaryWindow};

use super::pos_x_in_bounds;

pub fn update_preview(
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
            let texture_handle =
                asset_server.load(format!("{}.png", &next_generator.current_fruit.file_name));
            *handle = texture_handle;

            // update next preview
            let (_, mut next_sprite, mut next_handle) = next_preview.single_mut();
            next_sprite.custom_size = Some(Vec2::new(1.0, 1.0) * next_generator.next_fruit.size);
            let next_texture_handle =
                asset_server.load(format!("{}.png", &next_generator.next_fruit.file_name));
            *next_handle = next_texture_handle;
            next_generator.preview_updated();
        }
    }
}
