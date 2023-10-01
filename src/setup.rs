use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::{
        CONTAINER_BASE_OFFSET, CONTAINER_HEIGHT, CONTAINER_THICKNESS, CONTAINER_WIDTH, KNOWN_TYPES,
        NEXT_PREVIEW_LABEL_SIZE, NEXT_PREVIEW_OFFSET, SCREEN_HEIGHT, SCREEN_WIDTH, SPAWN_HEIGHT,
    },
    resources::{GameAlreadySetUp, NextGenerator},
    AppState,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(
                OnEnter(AppState::InGame),
                (setup_container, setup_app_boundaries, setup_merge_guide),
            )
            // unlike the other setups, previews are thrown out in GameOver state, and recreated after starting over
            .add_systems(OnEnter(AppState::InGame), setup_preview)
            .add_systems(OnExit(AppState::InGame), cleanup_preview);
    }
}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Preview;

#[derive(Component)]
pub struct NextPreview;

#[derive(Component)]
pub struct PreviewPart;

fn setup_container(mut commands: Commands, game_already_set_up: Res<GameAlreadySetUp>) {
    if game_already_set_up.is_set_up {
        return;
    }

    let container_base = -SCREEN_HEIGHT / 2.0 + CONTAINER_BASE_OFFSET;
    /* Create the container. */
    commands.spawn((
        Collider::cuboid(CONTAINER_WIDTH / 2.0, CONTAINER_THICKNESS / 2.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CONTAINER_WIDTH, CONTAINER_THICKNESS)),
                color: Color::rgb(0.0, 0.0, 0.55),
                ..default()
            },
            transform: Transform::from_xyz(0.0, container_base, 0.0),
            ..default()
        },
    ));

    let wall_height = CONTAINER_HEIGHT + CONTAINER_THICKNESS;
    let wall_base = container_base + CONTAINER_HEIGHT / 2.0;
    commands.spawn((
        Collider::cuboid(CONTAINER_THICKNESS / 2.0, wall_height / 2.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CONTAINER_THICKNESS, wall_height)),
                color: Color::rgb(0.0, 0.0, 0.55),
                ..default()
            },
            transform: Transform::from_xyz(
                (CONTAINER_WIDTH + CONTAINER_THICKNESS) / 2.0,
                wall_base,
                0.0,
            ),
            ..default()
        },
    ));
    commands.spawn((
        Collider::cuboid(CONTAINER_THICKNESS / 2.0, wall_height / 2.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(CONTAINER_THICKNESS, wall_height)),
                color: Color::rgb(0.0, 0.0, 0.55),
                ..default()
            },
            transform: Transform::from_xyz(
                -(CONTAINER_WIDTH + CONTAINER_THICKNESS) / 2.0,
                wall_base,
                0.0,
            ),
            ..default()
        },
    ));
}

fn setup_app_boundaries(mut commands: Commands, game_already_set_up: Res<GameAlreadySetUp>) {
    if game_already_set_up.is_set_up {
        return;
    }

    commands.spawn((
        Collider::cuboid(SCREEN_WIDTH / 2.0, 1.0),
        TransformBundle::from(Transform::from_xyz(0.0, -SCREEN_HEIGHT / 2.0, 0.0)),
    ));

    commands.spawn((
        Collider::cuboid(SCREEN_WIDTH / 2.0, 1.0),
        TransformBundle::from(Transform::from_xyz(0.0, SCREEN_HEIGHT / 2.0, 0.0)),
    ));

    commands.spawn((
        Collider::cuboid(1.0, SCREEN_HEIGHT / 2.0),
        TransformBundle::from(Transform::from_xyz(-SCREEN_WIDTH / 2.0, 0.0, 0.0)),
    ));

    commands.spawn((
        Collider::cuboid(1.0, SCREEN_HEIGHT / 2.0),
        TransformBundle::from(Transform::from_xyz(SCREEN_WIDTH / 2.0, 0.0, 0.0)),
    ));
}

fn setup_merge_guide(
    mut commands: Commands,
    game_already_set_up: Res<GameAlreadySetUp>,
    asset_server: Res<AssetServer>,
) {
    if game_already_set_up.is_set_up {
        return;
    }

    let normalize_size = |size: f32| -> f32 {
        let max = 204.0;
        let min = 26.0;
        size.max((size - min) / (max - min) * size)
            .min(size / 2.5 + 20.0)
    };

    let mut offset: f32 = 0.0;
    for (size, file_name) in KNOWN_TYPES.into_iter() {
        let texture_handle = asset_server.load(file_name);
        let pos_x = CONTAINER_WIDTH / 2.0 + NEXT_PREVIEW_OFFSET + KNOWN_TYPES[10].0;
        let pos_y = (-SCREEN_HEIGHT / 2.0 + CONTAINER_BASE_OFFSET) + offset;
        let normalized_size = normalize_size(size);
        offset += normalized_size + 10.0;
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0) * normalized_size),
                ..default()
            },
            texture: texture_handle,
            transform: Transform::from_xyz(pos_x, pos_y, 0.0),
            ..default()
        },));
    }
}

fn setup_preview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    next_generator: Res<NextGenerator>,
) {
    let file_name = &next_generator.current_fruit.image_file_name;
    let texture_handle = asset_server.load(file_name);
    commands.spawn((
        Preview,
        PreviewPart,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0) * next_generator.current_fruit.size),
                ..default()
            },
            texture: texture_handle,
            transform: Transform::from_xyz(0.0, SPAWN_HEIGHT, 0.0),
            ..default()
        },
    ));

    let file_name = &next_generator.next_fruit.image_file_name;
    let texture_handle = asset_server.load(file_name);
    commands.spawn((
        NextPreview,
        PreviewPart,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0) * next_generator.next_fruit.size),
                ..default()
            },
            texture: texture_handle,
            transform: Transform::from_xyz(
                CONTAINER_WIDTH / 2.0 + NEXT_PREVIEW_OFFSET,
                CONTAINER_HEIGHT / 2.0,
                0.0,
            ),
            ..default()
        },
    ));

    commands
        .spawn((
            PreviewPart,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 50.0)),
                    color: Color::rgb(0.56, 1.0, 0.98),
                    ..default()
                },
                transform: Transform::from_xyz(
                    CONTAINER_WIDTH / 2.0 + NEXT_PREVIEW_OFFSET,
                    CONTAINER_HEIGHT / 2.0 + KNOWN_TYPES[5].0 / 2.0 + NEXT_PREVIEW_LABEL_SIZE / 2.0,
                    0.0,
                ),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "NEXT",
                    TextStyle {
                        font_size: NEXT_PREVIEW_LABEL_SIZE,
                        color: Color::BLACK,
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });
}

fn cleanup_preview(mut commands: Commands, preview_parts: Query<Entity, With<PreviewPart>>) {
    for preview_part in preview_parts.iter() {
        commands.entity(preview_part).despawn_recursive();
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
