use bevy::{prelude::*, window::PrimaryWindow};

use crate::constants::{
    NEXT_BG_COLOR, RESTART_HEIGHT, RESTART_POS_X, RESTART_POS_Y, RESTART_WIDTH, TEXT_COLOR,
};
use crate::helpers::{get_mouse_pos, mouse_pos_in_restart};
use crate::resources::{GameAlreadySetUp, ScoreTracker};
use crate::setup::{MainCamera, Score};
use crate::Fruit;

pub fn setup_restart(mut commands: Commands, game_already_set_up: Res<GameAlreadySetUp>) {
    if game_already_set_up.is_set_up {
        return;
    }
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(RESTART_WIDTH, RESTART_HEIGHT)),
                color: NEXT_BG_COLOR,
                ..default()
            },
            transform: Transform::from_xyz(RESTART_POS_X, RESTART_POS_Y, 0.0),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((Text2dBundle {
                text: Text::from_section(
                    "restart",
                    TextStyle {
                        font_size: 25.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            },));
        });
}

pub fn handle_restart(
    mouse_button_input: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut commands: Commands,
    mut score_tracker: ResMut<ScoreTracker>,
    fruits: Query<Entity, With<Fruit>>,
    mut score_query: Query<&mut Text, With<Score>>,
) {
    let mouse_pos = get_mouse_pos(q_windows, camera_q);

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(world_position) = mouse_pos {
            if mouse_pos_in_restart(world_position) {
                score_tracker.reset();
                let mut score = score_query.single_mut();
                score.sections[0].value = score_tracker.score.to_string();
                for fruit in fruits.iter() {
                    commands.entity(fruit).despawn_recursive();
                }
            }
        }
    }
}
