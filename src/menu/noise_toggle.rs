use bevy::{prelude::*, window::PrimaryWindow};

use crate::constants::{
    NEXT_BG_COLOR, NOISE_TOGGLE_HEIGHT, NOISE_TOGGLE_POS_X, NOISE_TOGGLE_POS_Y, NOISE_TOGGLE_WIDTH,
    TEXT_COLOR,
};
use crate::helpers::{get_mouse_pos, mouse_pos_in_noise_toggle};
use crate::resources::{GameAlreadySetUp, NoiseSetting};
use crate::setup::MainCamera;

#[derive(Component)]
pub struct NoiseText;

pub fn setup_noise_toggle(mut commands: Commands, game_already_set_up: Res<GameAlreadySetUp>) {
    if game_already_set_up.is_set_up {
        return;
    }
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(NOISE_TOGGLE_WIDTH, NOISE_TOGGLE_HEIGHT)),
                color: NEXT_BG_COLOR,
                ..default()
            },
            transform: Transform::from_xyz(NOISE_TOGGLE_POS_X, NOISE_TOGGLE_POS_Y, 0.0),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                NoiseText,
                Text2dBundle {
                    text: Text::from_section(
                        "f--- that noise",
                        TextStyle {
                            font_size: 25.0,
                            color: TEXT_COLOR,
                            ..default()
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    transform: Transform::from_translation(Vec3::Z),
                    ..default()
                },
            ));
        });
}

pub fn handle_noise_toggle(
    mouse_button_input: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut noise_text: Query<&mut Text, With<NoiseText>>,
    mut noise_setting: ResMut<NoiseSetting>,
) {
    let mouse_pos = get_mouse_pos(q_windows, camera_q);

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(world_position) = mouse_pos {
            if mouse_pos_in_noise_toggle(world_position) {
                if let Ok(mut text) = noise_text.get_single_mut() {
                    noise_setting.toggle();
                    if noise_setting.is_on {
                        text.sections[0].value = String::from("f--- that noise");
                    } else {
                        text.sections[0].value = String::from("gimme that noise");
                    }
                }
            }
        }
    }
}
