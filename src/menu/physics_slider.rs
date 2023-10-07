use bevy::{prelude::*, window::PrimaryWindow};

use crate::helpers::{get_mouse_pos, mouse_pos_in_slider};
use crate::resources::{GameAlreadySetUp, MassSetting};
use crate::setup::MainCamera;

use crate::constants::{
    NEXT_BG_COLOR, PREVIEW_HINT_COLOR, SLIDER_CONTAINER_HEIGHT, SLIDER_CONTAINER_SIDES,
    SLIDER_CONTAINER_WIDTH, SLIDER_POS_X, SLIDER_POS_Y, SLIDER_WIDTH, TEXT_COLOR, TRANSPARENT,
    YAGOO_SIZE,
};

#[derive(Component)]
pub struct Yagoo;

pub fn setup_slider(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_already_set_up: Res<GameAlreadySetUp>,
) {
    if game_already_set_up.is_set_up {
        return;
    }
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SLIDER_CONTAINER_WIDTH, SLIDER_CONTAINER_HEIGHT)),
                color: NEXT_BG_COLOR,
                ..default()
            },
            transform: Transform::from_xyz(SLIDER_POS_X, SLIDER_POS_Y, 0.0),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(SLIDER_WIDTH, 5.0)),
                    color: PREVIEW_HINT_COLOR,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            });

            builder.spawn((
                Yagoo,
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1.0, 1.0) * 50.0),
                        ..default()
                    },
                    texture: asset_server.load("yagoo.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 2.0),
                    ..default()
                },
            ));
        });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SLIDER_CONTAINER_WIDTH, SLIDER_CONTAINER_HEIGHT)),
                color: TRANSPARENT,
                ..default()
            },
            transform: Transform::from_xyz(
                SLIDER_POS_X,
                SLIDER_POS_Y + SLIDER_CONTAINER_HEIGHT,
                0.0,
            ),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "physics",
                    TextStyle {
                        font_size: 25.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SLIDER_CONTAINER_WIDTH, SLIDER_CONTAINER_HEIGHT)),
                color: TRANSPARENT,
                ..default()
            },
            transform: Transform::from_xyz(SLIDER_POS_X - 75.0, SLIDER_POS_Y + 45.0, 0.0),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "stable",
                    TextStyle {
                        font_size: 25.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SLIDER_CONTAINER_WIDTH, SLIDER_CONTAINER_HEIGHT)),
                color: TRANSPARENT,
                ..default()
            },
            transform: Transform::from_xyz(SLIDER_POS_X + 75.0, SLIDER_POS_Y + 45.0, 0.0),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "crazy",
                    TextStyle {
                        font_size: 25.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });
}

pub fn handle_slider_change(
    mouse_button_input: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut yagoo: Query<&mut Transform, With<Yagoo>>,
    mut mass_setting: ResMut<MassSetting>,
) {
    let yagoo_max: f32 = SLIDER_CONTAINER_WIDTH / 2.0 - YAGOO_SIZE / 2.0;
    let yagoo_min: f32 = -SLIDER_CONTAINER_WIDTH / 2.0 + YAGOO_SIZE / 2.0;
    let mouse_pos = get_mouse_pos(q_windows, camera_q);

    if mouse_button_input.pressed(MouseButton::Left) {
        if let Some(world_position) = mouse_pos {
            if mouse_pos_in_slider(world_position) {
                let offset = (SLIDER_CONTAINER_WIDTH - SLIDER_WIDTH) / 2.0;
                let max_x = SLIDER_CONTAINER_SIDES.1 - offset;
                let min_x = SLIDER_CONTAINER_SIDES.3 + offset;
                let slider_center = (SLIDER_CONTAINER_SIDES.1 - SLIDER_CONTAINER_SIDES.3) / 2.0;
                let mouse_x = match world_position.x {
                    x if x < slider_center => x.max(min_x),
                    x if x > slider_center => x.min(max_x),
                    _ => world_position.x,
                };
                let percentage = (mouse_x - min_x) / (max_x - min_x);
                let mut yagoo_transform = yagoo.single_mut();
                yagoo_transform.translation.x = (yagoo_max - yagoo_min) * percentage + yagoo_min;
                mass_setting.percentage = 1.0 - percentage;
            }
        }
    }
}
