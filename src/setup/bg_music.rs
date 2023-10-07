use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::resources::NoiseSetting;

#[derive(Component)]
pub struct BgAudio;

pub fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn start_music(music_controller: Query<&AudioSink, With<BgAudio>>) {
    if let Ok(sink) = music_controller.get_single() {
        sink.play();
    }
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
