use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::resources::{GameAlreadySetUp, NoiseSetting};

#[derive(Component)]
pub struct BgAudio;

pub fn setup_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_already_set_up: Res<GameAlreadySetUp>,
) {
    if game_already_set_up.is_set_up {
        return;
    }
    commands.spawn((
        BgAudio,
        AudioBundle {
            source: asset_server.load("audio/bg.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new_absolute(0.1),
                paused: true,
                ..default()
            },
        },
    ));
}

pub fn on_music_setting_change(
    noise_setting: Res<NoiseSetting>,
    music_controller: Query<&AudioSink, With<BgAudio>>,
) {
    if noise_setting.is_changed() {
        if let Ok(sink) = music_controller.get_single() {
            if noise_setting.is_on {
                sink.play();
            } else {
                sink.pause();
            }
        }
    }
}
