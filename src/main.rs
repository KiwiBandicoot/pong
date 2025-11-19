use bevy::{color::palettes::css::{DARK_GRAY, BLUE, RED}, prelude::*, window::WindowResolution};
use rand::Rng;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
struct Score(HashMap<Player, u32>);

const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HEIGHT: f32 = 720.;
const BALL_RADIUS: f32 = 25.;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins
    .set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.init_resource::<Score>();
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(true));
    app.add_event::<GameEvents>();
    app.add_systems(Startup, (spawn_score, spawn_camera, spawn_players));
    app.add_systems(Startup, (spawn_ball, spawn_border));
    app.add_systems(Update, move_paddle);
    app.add_systems(Update, detect_reset);
    app.add_systems(Update, ball_hit);
    app.add_systems(PostUpdate, (reset_ball, score));
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
    Player1,
    Player2,
}

impl Player {
    fn start_speed(&self) -> Velocity {
        match self {
            Player::Player1 => Velocity::linear(Vect::new(100., 0.)),
            Player::Player2 => Velocity::linear(Vect::new(-100., 0.)),
        }
    }

    fn get_color(&self) -> Color {
        match self {
            Player::Player1 => RED.into(),
            Player::Player2 => BLUE.into(),
        }
    }
}

fn spawn_border(mut commands: Commands) {
    // Top border
    commands.spawn((
        Transform::from_translation(Vec3::new(0., WINDOW_HEIGHT / 2., 0.)),
        RigidBody::Fixed,
        Collider::cuboid(WINDOW_WIDTH / 2., 3.)
    ));
    
    // Bottom border
    commands.spawn((
        Transform::from_translation(Vec3::new(0., -WINDOW_HEIGHT / 2., 0.)),
        RigidBody::Fixed,
        Collider::cuboid(WINDOW_WIDTH / 2., 3.)
    ));
    
    // Right border (Player 1 goal)
    commands.spawn((
        Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2., 0., 0.)),
        RigidBody::Fixed,
        Collider::cuboid(3., WINDOW_HEIGHT / 2.),
        Player::Player1,
        Sensor,
    ));
    
    // Left border (Player 2 goal)
    commands.spawn((
        Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2., 0., 0.)),
        RigidBody::Fixed,
        Collider::cuboid(3., WINDOW_HEIGHT / 2.),
        Player::Player2,
        Sensor,
    ));
}

const PWIDTH: f32 = 10.;
const PHIGTH: f32 = 150.;

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Player::Player1.get_color(),
            custom_size: Some(Vec2::new(PWIDTH, PHIGTH)),
            ..Default::default()
        },
    Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2. + 20., 0., 0.)),
    Paddle {
        move_up: KeyCode::KeyW,
        move_down: KeyCode::KeyS,
    },
    Player::Player1,
    RigidBody::KinematicPositionBased,
    Collider::cuboid(5., 75.),
    ));

    commands.spawn((
        Sprite {
            color: Player::Player2.get_color(),
            custom_size: Some(Vec2::new(PWIDTH, PHIGTH)),
            ..Default::default()
        },
    Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2. - 20., 0., 0.)),
    Paddle {
        move_up: KeyCode::ArrowUp,
        move_down: KeyCode::ArrowDown,
    },
    Player::Player2,
    RigidBody::KinematicPositionBased,
    Collider::cuboid(5., 75.),
    ));
}


fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += 100. * time.delta_secs();
            pos.translation.y = pos.translation.y.clamp((-WINDOW_HEIGHT / 2.) + 75., (WINDOW_HEIGHT / 2.) - 75.);
        }

        if input.pressed(settings.move_down) {
            pos.translation.y -= 100. * time.delta_secs();
            pos.translation.y = pos.translation.y.clamp((-WINDOW_HEIGHT / 2.) + 75., (WINDOW_HEIGHT / 2.) - 75.);
        }
    }
}

#[derive(Component)]
struct Ball;

fn spawn_ball(
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    let vx = if rng.gen_bool(0.5) { 100. } else { -100. };
    let vy = rng.gen_range(-50.0..50.0);

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(BALL_RADIUS * 2., BALL_RADIUS * 2.)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0., 0., 1.)),
        Ball,
        RigidBody::Dynamic,
        Collider::ball(BALL_RADIUS),
        Velocity::linear(Vec2::new(vx, vy)),
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        ActiveEvents::COLLISION_EVENTS,
    ));
}


fn ball_hit(
    paddles: Query<&Player, With<Paddle>>,
    mut balls: Query<(&CollidingEntities, &mut Sprite), With<Ball>>
) {
    for (hits, mut sprite) in &mut balls {
        for hit in hits.iter() {
            if let Ok(player) = paddles.get(hit) {
                sprite.color = player.get_color();
                return;
            }
        }
    }
}

fn detect_reset(
    input: Res<ButtonInput<KeyCode>>,
    balls: Query<&CollidingEntities, With<Ball>>,
    goals: Query<&Player, With<Sensor>>,
    mut game_events: EventWriter<GameEvents>,
) {
    if input.just_pressed(KeyCode::Space) {
        let player = if rand::thread_rng().gen::<bool>() {
            Player::Player1
        } else {
            Player::Player2
        };
        game_events.write(GameEvents::ResetBall(player));
        return;
    }
    for ball in &balls {
        for hit in ball.iter() {
            if let Ok(player) = goals.get(hit) {
                game_events.write(GameEvents::ResetBall(*player));
                game_events.write(GameEvents::GainPoint(*player));
            }
        }
    }
}

#[derive(Event)]
enum GameEvents {
    ResetBall(Player),
    GainPoint(Player),
}

fn reset_ball(
    mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut game_events: EventReader<GameEvents>,
) {
    for events in game_events.read() {
        match events {
            GameEvents::ResetBall(player) => {
                for (mut transform, mut velocity) in &mut balls {
                    transform.translation = Vec3::new(0., 0., 1.);
                    *velocity = player.start_speed();
                }
            },
            _ => {}
        }
    }
}

fn spawn_score(
    mut commands: Commands,
) {
    commands.spawn((Node {
        position_type: PositionType::Absolute,
        margin: UiRect::horizontal(Val::Auto),
        top: Val::ZERO,
        padding: UiRect::horizontal(Val::Px(20.)),
        display: Display::Grid,
        grid_template_columns: vec![GridTrack::flex(1.), GridTrack::auto(), GridTrack::flex(1.)],
        ..Default::default()
    }, BackgroundColor(DARK_GRAY.into()))).with_children(|p| {

        p.spawn((Text::new("0"), TextFont {
            font_size: 100.,
            ..Default::default()
        }, TextLayout::new_with_justify(JustifyText::Center),
        Player::Player1));

        p.spawn((Text::new("|"), TextFont {
            font_size: 100.,
            ..Default::default()
        }));

        p.spawn((Text::new("0"), TextFont {
            font_size: 100.,
            ..Default::default()
        }, TextLayout::new_with_justify(JustifyText::Center),
        Player::Player2));
    });
}

fn score(
    mut events: EventReader<GameEvents>,
    mut score_text: Query<(&mut Text, &Player)>,
    mut score: ResMut<Score>,
) {
    for event in events.read() {
        match event {
            GameEvents::GainPoint(player) => {
                *score.0.entry(*player).or_default() += 1;
                let score = score.0.get(player).cloned().unwrap_or(0);
                for (mut text, owner) in &mut score_text {
                    if owner != player {continue;}
                    **text = score.to_string();
                    break;
                }
            },
            GameEvents::ResetBall(_) => {}
        }
    }
}