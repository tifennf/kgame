use bevy::{app::Plugin, prelude::Resource};

use crate::{
    dot::{DotColor, DotStorage},
    TILEMAP_SIZE,
};

// Game internal state
#[derive(Resource)]
pub struct GameState {
    pub turn: u32,
    pub dot_color: DotColor,
    pub dot_storage: DotStorage,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            turn: 1,
            dot_color: DotColor::BLUE,
            dot_storage: DotStorage::empty(TILEMAP_SIZE * TILEMAP_SIZE),
        }
    }
}

impl GameState {
    pub fn next_turn(&mut self) {
        self.dot_color = match self.dot_color {
            DotColor::RED => DotColor::BLUE,
            DotColor::BLUE => DotColor::RED,
        };

        self.turn += 1;
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<GameState>();
    }
}

pub fn check_winner() {}
