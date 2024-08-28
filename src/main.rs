mod game_state;
mod grid;
mod node;
mod utils;

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use game_state::GameStatePlugin;
use grid::GridPlugin;
use node::spawn_node_on_click;
use utils::UtilsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UtilsPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(GridPlugin)
        .add_plugins(TilemapPlugin)
        .add_systems(Update, spawn_node_on_click)
        .run();
}
