use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_egui::EguiPlugin;

mod foob;
mod game_data;
mod game_state;
mod screen;
mod settings;
mod snek;

use foob::*;
use game_data::*;
use game_state::*;
use screen::*;
use settings::*;
use snek::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snek!".to_string(),
            width: 900.0,
            height: 900.0,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(GameData {
            game_state: GameState::Menu,
            score: 0,
        })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(Settings::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(SnekPlugin)
        .add_plugin(FoobPlugin)
        .add_plugin(GameStatePlugin)
        .add_plugin(ScreenPlugin)
        .add_startup_system(setup)
        .add_system(settings_ui)
        .add_system(background_color)
        .run();
}

#[derive(Component)]
struct BackgroundTile;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default()
                .with_scale(Vec3::splat(890.))
                .with_translation(Vec3::new(0., 0., 0.)),
            material: materials.add(ColorMaterial::from(Color::DARK_GRAY)),
            ..default()
        })
        .insert(BackgroundTile);
}

fn background_color(
    mut background_border: ResMut<ClearColor>,
    mut background_query: Query<&mut Handle<ColorMaterial>, With<BackgroundTile>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<Settings>,
) {
    for tile in background_query.iter_mut() {
        let mut color_mat = materials.get_mut(&tile).unwrap();
        color_mat.color = Color::rgba_u8(
            settings.background.r(),
            settings.background.g(),
            settings.background.b(),
            settings.background.a(),
        );
    }

    if settings.wraparound {
        background_border.0 = Color::rgba_u8(
            settings.background.r(),
            settings.background.g(),
            settings.background.b(),
            settings.background.a(),
        );
    } else {
        background_border.0 = Color::rgba_u8(
            255 - settings.background.r(),
            255 - settings.background.g(),
            255 - settings.background.b(),
            255 - settings.background.a(),
        );
    }
}
