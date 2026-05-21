use bevy::prelude::*;

use crate::{menu::{MenuState, Winner}, Player, Score};

#[derive(Component)]
pub enum GameOverButton {
    PlayAgain,
    Menu,
}

#[derive(Component)]
pub struct GameOverUI;

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    winner: Res<Winner>,
    score: Res<Score>,
) {
    let winner_color = match winner.0 {
        Some(Player::Player1) => Color::srgb(0.0, 0.0, 1.0),
        Some(Player::Player2) => Color::srgb(1.0, 0.0, 0.0),
        None => Color::WHITE,
    };
    let winner_name = match winner.0 {
        Some(Player::Player1) => "Blue",
        Some(Player::Player2) => "Red",
        None => "Game",
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,


                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(18.0),
                ..Default::default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                TextFont {

                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 80.0,
                    ..Default::default()
                },
                Node {
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((

                    TextSpan::new(winner_name),
                    TextColor(winner_color),
                    TextFont {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 80.0,
                        ..Default::default()
                    },
                ));
                parent.spawn((
                    TextSpan::new(" Wins"),
                    TextColor(Color::WHITE),
                    TextFont {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 80.0,
                        ..Default::default()
                    },
                ));
            });

            parent.spawn((
                Text::new(format!("Final score  {}  -  {}", score.player1, score.player2)),
                TextFont {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 28.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    GameOverButton::PlayAgain,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play Again"),
                        TextFont {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    GameOverButton::Menu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Main Menu"),
                        TextFont {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn cleanup_game_over_menu(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn game_over_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &GameOverButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match button {
                GameOverButton::PlayAgain => app_state.set(MenuState::InGame),
                GameOverButton::Menu => app_state.set(MenuState::MainMenu),
            },
        Interaction::Hovered => *color = Color::srgb(0.3, 0.3, 0.3).into(),
        Interaction::None => *color = Color::srgb(0.5, 0.5, 0.5).into(),       }
    }
}