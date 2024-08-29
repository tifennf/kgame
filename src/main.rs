mod dot;
mod game;
mod grid;
mod utils;

use bevy::prelude::*;

use bevy_ecs_tilemap::TilemapPlugin;
use dot::spawn_dot_on_click;
use game::GameStatePlugin;
use grid::GridPlugin;
use utils::UtilsPlugin;

pub const TILEMAP_SIZE: u32 = 100; // tilemap is square matrix shape
pub const TILE_ASSET_SIZE: f32 = 32.0; // px
pub const TILE_ASSET_PATH: &str = "tiles.png";

fn main() {
    App::new()
        // .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(UtilsPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(GridPlugin)
        .add_plugins(TilemapPlugin)
        .add_systems(Update, spawn_dot_on_click)
        .run();
}
