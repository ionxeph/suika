use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::RigidBody;

use crate::constants::{BG_COLOR, SCREEN_HEIGHT, TEXT_COLOR};
use crate::resources::{Fruit, GameAlreadySetUp, ScoreTracker};
use crate::setup::{MainCamera, Score};
use crate::AppState;

use crate::helpers::get_mouse_pos;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_menu)
            .add_systems(Update, menu_system.run_if(in_state(AppState::StartMenu)))
            .add_systems(OnExit(AppState::StartMenu), cleanup_menu)
            .add_systems(OnEnter(AppState::GameOverMenu), setup_game_over)
            .add_systems(Update, menu_system.run_if(in_state(AppState::GameOverMenu)))
            .add_systems(
                OnExit(AppState::GameOverMenu),
                (cleanup_menu, cleanup_fruits),
            );
    }
}

#[derive(Component)]
pub struct MenuItem;

fn setup_menu(mut commands: Commands) {
    commands.spawn((
        MenuItem,
        Text2dBundle {
            text: Text::from_section(
                "click anywhere to begin",
                TextStyle {
                    font_size: 30.0,
                    color: TEXT_COLOR,
                    ..default()
                },
            )
            .with_alignment(TextAlignment::Center),
            ..default()
        },
    ));
}

fn cleanup_menu(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    for menu_item in menu_items.iter() {
        commands.entity(menu_item).despawn_recursive();
    }
}

fn setup_game_over(
    mut commands: Commands,
    mut game_already_set_up: ResMut<GameAlreadySetUp>,
    mut fruits: Query<&mut RigidBody, With<Fruit>>,
) {
    game_already_set_up.is_set_up = true;
    commands
        .spawn((
            MenuItem,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    color: BG_COLOR,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, SCREEN_HEIGHT / 2.0 - 100.0, 0.0),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((Text2dBundle {
                text: Text::from_section(
                    "click anywhere to restart",
                    TextStyle {
                        font_size: 30.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            },));
        });

    commands
        .spawn((
            MenuItem,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    color: BG_COLOR,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, SCREEN_HEIGHT / 2.0 - 200.0, 0.0),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((Text2dBundle {
                text: Text::from_section(
                    "GAME OVER",
                    TextStyle {
                        font_size: 50.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            },));
        });

    for mut fruit in fruits.iter_mut() {
        *fruit = RigidBody::Fixed;
    }
}

fn cleanup_fruits(
    mut commands: Commands,
    mut score_tracker: ResMut<ScoreTracker>,
    fruits: Query<Entity, With<Fruit>>,
    mut score_query: Query<&mut Text, With<Score>>,
) {
    score_tracker.reset();
    let mut score = score_query.single_mut();
    score.sections[0].value = score_tracker.score.to_string();
    for fruit in fruits.iter() {
        commands.entity(fruit).despawn_recursive();
    }
}

fn menu_system(
    mut next_state: ResMut<NextState<AppState>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    let mouse_pos = get_mouse_pos(q_windows, camera_q);

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(_world_position) = mouse_pos {
            next_state.set(AppState::InGame);
        }
    }
}
