use bevy::prelude::*;

#[derive(Component)]
pub enum MenuButton {
    Start,
    Quit,
}

#[derive(Component)]
pub struct MainMenuUI;

#[derive(States, PartialEq, Eq, Debug, Hash, Clone, Copy, Default)]
pub enum MenuState {
    #[default]
    MainMenu,
    InGame,
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..Default::default()
            },
            MainMenuUI,
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
                    TextSpan::new("Red "),
                    TextColor(Color::srgb(1.0, 0.0, 0.0)),
                    TextFont {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 80.0,
                    ..Default::default()
                },
                ));
                parent.spawn((
                    TextSpan::new("Vs "),
                    TextColor(Color::WHITE),
                    TextFont {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 80.0,
                    ..Default::default()
                },
                ));
                parent.spawn((
                    TextSpan::new("Blue"),
                    TextColor(Color::srgb(0.0, 0.0, 1.0)),
                    TextFont {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 80.0,
                    ..Default::default()
                },
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
                MenuButton::Start,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Start"),
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
                MenuButton::Quit,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Quit"),
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
        
pub fn menu_button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, &MenuButton),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_state: ResMut<NextState<MenuState>>,
        mut exit: EventWriter<AppExit>,
    ) {
        for (interaction, mut color, button) in &mut interaction_query {
            match *interaction {
        Interaction::Pressed => match button {
            MenuButton::Start => app_state.set(MenuState::InGame),
            MenuButton::Quit => {
                exit.write(AppExit::Success);
            }
        },
        Interaction::Hovered => *color = Color::srgb(0.3, 0.3, 0.3).into(),
        Interaction::None => *color = Color::srgb(0.5, 0.5, 0.5).into(),
    }}
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
