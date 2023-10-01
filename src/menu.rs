use bevy::{prelude::*, window::PrimaryWindow};

use crate::{setup::MainCamera, AppState};

use crate::helpers::get_mouse_pos;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::StartMenu), setup_menu)
            .add_systems(Update, menu_system.run_if(in_state(AppState::StartMenu)))
            .add_systems(OnExit(AppState::StartMenu), cleanup_menu);
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
                    color: Color::BLACK,
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
        commands.entity(menu_item).despawn();
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
