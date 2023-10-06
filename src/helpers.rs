use bevy::{prelude::*, window::PrimaryWindow};

use crate::{constants::SLIDER_CONTAINER_SIDES, setup::MainCamera};

pub fn get_mouse_pos(
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

pub fn mouse_pos_in_slider(mouse_pos: Vec2) -> bool {
    let x = mouse_pos.x;
    let y = mouse_pos.y;
    y <= SLIDER_CONTAINER_SIDES.0
        && x <= SLIDER_CONTAINER_SIDES.1
        && y >= SLIDER_CONTAINER_SIDES.2
        && x >= SLIDER_CONTAINER_SIDES.3
}
