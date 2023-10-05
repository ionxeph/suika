use bevy::prelude::*;

use crate::constants::{
    NEXT_BG_COLOR, PREVIEW_HINT_COLOR, SCREEN_HEIGHT, SCREEN_WIDTH, TEXT_COLOR, TRANSPARENT,
};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pos_x = SCREEN_WIDTH / 2.0 - 150.0;
    let pox_y = -SCREEN_HEIGHT / 2.0 + 100.0;
    let width = 250.0;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, 80.0)),
                color: NEXT_BG_COLOR,
                ..default()
            },
            transform: Transform::from_xyz(pos_x, pox_y, 0.0),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(width - 25.0, 5.0)),
                    color: PREVIEW_HINT_COLOR,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            });

            builder.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0) * 50.0),
                    ..default()
                },
                texture: asset_server.load("yagoo.png"),
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                ..default()
            });
        });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, 80.0)),
                color: TRANSPARENT,
                ..default()
            },
            transform: Transform::from_xyz(pos_x, pox_y + 80.0, 0.0),
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
                custom_size: Some(Vec2::new(width, 80.0)),
                color: TRANSPARENT,
                ..default()
            },
            transform: Transform::from_xyz(pos_x - 75.0, pox_y + 45.0, 0.0),
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
                custom_size: Some(Vec2::new(width, 80.0)),
                color: TRANSPARENT,
                ..default()
            },
            transform: Transform::from_xyz(pos_x + 75.0, pox_y + 45.0, 0.0),
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

pub fn handle_slider_change() {
    let yagoo_max: f32 = 250.0 / 2.0 - 25.0;
    let yagoo_min: f32 = -250.0 / 2.0 + 25.0;

    // TODO: implement drag, and add a resource to tie the slider value to mass
}
