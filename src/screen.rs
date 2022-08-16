use bevy::prelude::*;
use bevy::render::view::Visibility;
pub struct ScreenPlugin;

use crate::{game_data::GameData, game_state::GameState};

#[derive(Component)]
pub struct ShowOnScreen(GameState);

#[derive(Component)]
pub struct ShowLiveScore;

#[derive(Component)]
pub struct ShowEndScore;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_screen)
            .add_system(screen_system)
            .add_system(show_score);
    }
}

fn setup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(600.0), Val::Percent(20.0)),
                position: UiRect {
                    top: Val::Px(0.0),
                    left: Val::Px(150.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Px(0.0),
                    bottom: Val::Auto,
                },
                ..default()
            },
            //color: Color::rgba(0.15, 0., 0.15, 0.5).into(),
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Rust Snek!",
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Regular.ttf"),
                        font_size: 128.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(0.0),
                        top: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                }),
            );
        })
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Press W, A, S, D or the arrow keys to start...",
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Regular.ttf"),
                        font_size: 32.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position: UiRect {
                        top: Val::Px(150.0),
                        ..default()
                    },
                    ..default()
                }),
            );
        })
        .insert(ShowOnScreen(GameState::Menu));

    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Score: 00",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 32.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            )
            .with_text_alignment(TextAlignment::CENTER)
            .with_style(Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: UiRect {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(ShowOnScreen(GameState::Playing))
        .insert(ShowLiveScore);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(600.0), Val::Percent(20.0)),
                position: UiRect {
                    top: Val::Px(0.0),
                    left: Val::Px(150.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Px(0.0),
                    bottom: Val::Auto,
                },
                ..default()
            },
            //color: Color::rgba(0.15, 0., 0.15, 0.5).into(),
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Game Over",
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Regular.ttf"),
                        font_size: 128.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(0.0),
                        top: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                }),
            );
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(
                    TextBundle::from_section(
                        "Your Score: 00",
                        TextStyle {
                            font: asset_server.load("fonts/Roboto-Regular.ttf"),
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )
                    .with_text_alignment(TextAlignment::CENTER)
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position: UiRect {
                            top: Val::Px(150.0),
                            ..default()
                        },
                        ..default()
                    }),
                )
                .insert(ShowEndScore);
        })
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Press space to restart",
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Regular.ttf"),
                        font_size: 24.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position: UiRect {
                        top: Val::Px(175.0),
                        ..default()
                    },
                    ..default()
                }),
            );
        })
        .insert(ShowOnScreen(GameState::GameOver));
}

fn screen_system(
    game_data: Res<GameData>,
    children_query: Query<&Children>,
    mut visible_query: Query<&mut Visibility>,
    ui_elements: Query<(&ShowOnScreen, Entity)>,
) {
    for (show_on_screen, entity) in ui_elements.iter() {
        if show_on_screen.0 == game_data.game_state {
            set_visible_recursive(true, entity, &mut visible_query, &children_query);
        } else {
            set_visible_recursive(false, entity, &mut visible_query, &children_query);
        }
    }
}

fn set_visible_recursive(
    is_visible: bool,
    entity: Entity,
    visible_query: &mut Query<&mut Visibility>,
    children_query: &Query<&Children>,
) {
    if let Ok(mut visible) = visible_query.get_mut(entity) {
        visible.is_visible = is_visible;
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            set_visible_recursive(is_visible, *child, visible_query, children_query);
        }
    }
}

fn show_score(
    mut score_query: Query<&mut Text, With<ShowLiveScore>>,
    mut end_score_query: Query<&mut Text, (With<ShowEndScore>, Without<ShowLiveScore>)>,
    score: Res<GameData>,
) {
    for mut text in score_query.iter_mut() {
        text.sections[0].value = format!("Score: {}", score.score);
    }

    for mut text in end_score_query.iter_mut() {
        text.sections[0].value = format!("Your Score: {}", score.score);
    }
}
