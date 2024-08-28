mod game_state;
mod grid;
mod node;
mod utils;

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use game_state::GameStatePlugin;
use grid::GridPlugin;
use node::{spawn_dot_on_click, Dot};
use utils::UtilsPlugin;

pub const N: u32 = 16;
pub const TILE_ASSET_SIZE: f32 = 32.0; // px
pub const TILE_ASSET_PATH: &str = "tiles.png";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UtilsPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(GridPlugin)
        .add_plugins(TilemapPlugin)
        .add_systems(Update, spawn_dot_on_click)
        .run();
}
