mod api;
mod dot;
mod game;
mod grid;
mod utils;

use std::sync::Arc;

use api::{
    channel::{BevyMessage, ChannelManager, ServerMessage},
    handler::get_game_state,
};
use axum::{routing::get, Router};
use bevy::{prelude::*, tasks::IoTaskPool};

use bevy_ecs_tilemap::TilemapPlugin;
use dot::spawn_dot_on_click;
use flume::{Receiver, Sender};
use game::GameStatePlugin;
use grid::GridPlugin;
use utils::UtilsPlugin;

pub const TILEMAP_SIZE: u32 = 100; // tilemap is square matrix shape
pub const TILE_ASSET_SIZE: f32 = 32.0; // px
pub const TILE_ASSET_PATH: &str = "tiles.png";
pub const KAING_VALUE: u32 = 5; // how many aligned dot to win

#[tokio::main]
async fn main() {
    // spawn a thread for api server

    let (tx_server, rx_server) = flume::unbounded::<ServerMessage>();
    let (tx_bevy, rx_bevy) = flume::unbounded::<BevyMessage>();

    tokio::spawn(async move {
        let state = Arc::new(ChannelManager {
            tx: tx_server,
            rx: rx_bevy,
        });

        let get = get(get_game_state);
        let app = Router::new().route("/", get).with_state(state);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        axum::serve(listener, app).await.unwrap();
    });

    // Bevy need to run on the main thread
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UtilsPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(GridPlugin)
        .add_plugins(TilemapPlugin)
        .add_systems(Update, spawn_dot_on_click)
        .insert_resource(ChannelManager {
            tx: tx_bevy,
            rx: rx_server,
        })
        .run();
}
