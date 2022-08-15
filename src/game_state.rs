use bevy::prelude::*;

use crate::game_data::{self, GameData};
use crate::snek::*;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_gamestate_system);
    }
}

fn handle_gamestate_system(
    mut game_data: ResMut<GameData>, 
    keyboard_input: Res<Input<KeyCode>>,
    commands: Commands,
) {
    match game_data.game_state {
        GameState::Menu => {
            if keyboard_input.just_pressed(KeyCode::D)
                || keyboard_input.just_pressed(KeyCode::Right)
                || keyboard_input.just_pressed(KeyCode::A)
                || keyboard_input.just_pressed(KeyCode::Left)
                || keyboard_input.just_pressed(KeyCode::W)
                || keyboard_input.just_pressed(KeyCode::Up)
                || keyboard_input.just_pressed(KeyCode::S)
                || keyboard_input.just_pressed(KeyCode::Down)
            {
                game_data.game_state = GameState::Playing;
            }
        }
        GameState::Playing => {}
        GameState::GameOver => {
            if keyboard_input.just_pressed(KeyCode::Space)
            {
                game_data.game_state = GameState::Menu;
            }
        }
    }
}
