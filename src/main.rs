mod grid;

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use grid::GridPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GridPlugin)
        .add_plugins(TilemapPlugin)
        .run();
}
