use bevy::{
    app::Plugin,
    prelude::{Res, Resource},
};

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
    pub open: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            turn: 1,
            dot_color: DotColor::BLUE,
            dot_storage: DotStorage::empty(TILEMAP_SIZE as usize),
            open: true,
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

    pub fn check_winner(&mut self) {}
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<GameState>();
    }
}
