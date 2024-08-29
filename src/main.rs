mod dot;
mod game;
mod grid;
mod utils;

use bevy::prelude::*;

use utils::UtilsPlugin;

pub const TILEMAP_SIZE: u32 = 10; // tilemap is square matrix shape
pub const TILE_ASSET_SIZE: f32 = 32.0; // px
pub const TILE_ASSET_PATH: &str = "tiles.png";

fn main() {
    App::new()
        // .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(UtilsPlugin)
        // .add_plugins(GameStatePlugin)
        // .add_plugins(GridPlugin)
        // .add_plugins(TilemapPlugin)
        // .add_systems(Update, spawn_dot_on_click)
        .add_systems(Startup, test)
        .run();
}

fn test(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load(TILE_ASSET_PATH),

        ..default()
    });
}
