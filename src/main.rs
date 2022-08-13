use bevy::prelude::*;

mod foob;
mod game_data;
mod game_state;
mod snek;

use foob::*;
use game_data::*;
use game_state::*;
use snek::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snek!".to_string(),
            width: 900.0,
            height: 900.0,
            ..Default::default()
        })
        .insert_resource(GameData {
            game_state: GameState::Menu,
            score: 0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SnekPlugin)
        .add_plugin(FoobPlugin)
        .add_plugin(GameStatePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
