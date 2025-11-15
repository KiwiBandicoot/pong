use bevy::prelude::*;

const HEIGHT: f64 = 640.0;
const WIDTH: f64 = 480.0;

struct Paddle {
    width: f64,
    height: f64,
}

struct Ball {
    width: f64,
    height: f64,
    dx: f64,
    dy: f64,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.run();
}

