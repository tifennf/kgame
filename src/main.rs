mod grid;
mod node;

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use grid::GridPlugin;
use node::handle_click;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GridPlugin)
        .add_plugins(TilemapPlugin)
        .add_systems(Update, handle_click)
        .run();
}
