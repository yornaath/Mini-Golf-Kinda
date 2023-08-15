use bevy::{prelude::*, transform};

pub fn game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            z_index: ZIndex::Global(1),
            background_color: Color::BLACK.into(),
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: Val::Percent(20.0),
                bottom: Val::Percent(20.0),
                width: Val::Percent(60.),
                height: Val::Percent(60.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Game\nOver!",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 100.0,
                        color: Color::RED,
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                style: Style {
                    margin: UiRect::bottom(Val::Px(10.)),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

pub fn game_won(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            z_index: ZIndex::Global(1),
            background_color: Color::GREEN.into(),
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: Val::Percent(20.0),
                bottom: Val::Percent(20.0),
                width: Val::Percent(60.),
                height: Val::Percent(60.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Tada!",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                style: Style {
                    margin: UiRect::bottom(Val::Px(10.)),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
