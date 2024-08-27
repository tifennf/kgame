mod grid;
mod node;
mod utils;

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use grid::GridPlugin;
use node::handle_click;
use utils::{camera_movement, update_cursor_pos, CursorPos, UtilsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UtilsPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(TilemapPlugin)
        .add_systems(Update, handle_click)
        .run();
}
