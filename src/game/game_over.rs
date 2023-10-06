use crate::AppState;
use bevy::prelude::*;

use crate::constants::{CONTAINER_THICKNESS, CONTAINER_WIDTH, GAME_OVER_HEIGHT};

use super::Alive;
pub fn check_game_over(
    positions: Query<&Transform, With<Alive>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for position in positions.iter() {
        if position.translation.y > GAME_OVER_HEIGHT {
            next_state.set(AppState::GameOverMenu);
        }
        if position.translation.x > CONTAINER_WIDTH / 2.0 + CONTAINER_THICKNESS
            || position.translation.x < -CONTAINER_WIDTH / 2.0 - CONTAINER_THICKNESS
        {
            next_state.set(AppState::GameOverMenu);
        }
    }
}
