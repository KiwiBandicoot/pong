use bevy::prelude::*;

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_startup_system(Startup, spawn_camera);
    app.run();
}

