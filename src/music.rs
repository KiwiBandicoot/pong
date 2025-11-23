use bevy::prelude::*;

pub fn play_menu_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/01 Calm 1.ogg")),
        PlaybackSettings::LOOP,
    ));
}

