use bevy::{prelude::*, ui::FocusPolicy};
pub struct ScreenPlugin;

use crate::{game_state::GameState, game_data::GameData};

#[derive(Component)]
pub struct ShowOnScreen(GameState);

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_screen)
        .add_system(screen_system);
    }
}

fn setup_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(600.0), Val::Percent(20.0)),
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
            color: Color::rgba(0.15, 0., 0.15, 0.5).into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Rust Snek!",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 128.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            )
            .with_text_alignment(TextAlignment::CENTER)
            .with_style(Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Px(0.0), 
                    top: Val::Px(10.0),
                    ..default()
                },
                ..default()
        }));
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Press W, A, S, D or the arrow keys to start!",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 32.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            )
            .with_text_alignment(TextAlignment::CENTER)
            .with_style(Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Auto, 
                    top: Val::Px(40.0),
                    ..default()
                },
                ..default()
            })
        );
    })
    .insert(ShowOnScreen(GameState::Menu));
}

fn screen_system(
    game_data: Res<GameData>,
    mut ui_elements: Query<(&ShowOnScreen)>,
) {
    for (show_on_screen) in ui_elements.iter_mut() {
        if show_on_screen.0 == game_data.game_state {
            
        } else {
            
        }
    }
}