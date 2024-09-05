use std::future::IntoFuture;

use bevy::{
    prelude::{EventWriter, Query, Res, Resource, Transform},
    tasks::IoTaskPool,
};
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::TileStorage,
};
use flume::{Receiver, Sender};

use crate::{
    game::Game,
    grid::{compute_tile_center_map, TileClickEvent},
};

use super::handler::ApiDot;

// from bevy to server
#[derive(Clone)]
pub enum BevyMessage {
    State(Game),
    DotPlaced,
    InvalidDotPosition,
}

// from server to bevy
#[derive(Clone)]
pub enum ServerMessage {
    GetState,
    PlaceDot(ApiDot),
}

#[derive(Resource, Clone)]
pub struct ChannelManager<T, L> {
    pub tx: Sender<T>,
    pub rx: Receiver<L>,
}

// communicate with server using channel in order to perform player's actions
pub fn handle_bevy_channel(
    chan: Res<ChannelManager<BevyMessage, ServerMessage>>,
    gstate: Res<Game>,
    q_tilemap: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    mut ev_tile_click: EventWriter<TileClickEvent>,
) {
    while let Ok(msg) = chan.rx.try_recv() {
        match msg {
            ServerMessage::GetState => {
                let s = gstate.as_ref().clone();
                if let Err(e) = chan.tx.try_send(BevyMessage::State(s)) {
                    println!("{}", e);
                }
            }
            ServerMessage::PlaceDot(dot) => {
                let tile_pos = dot.get_tilepos();

                for (_, grid_size, map_type, tile_storage, map_transform) in q_tilemap.iter() {
                    if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                        // compute tile center, 2 spaces involved: world, tilemap
                        let tile_center =
                            compute_tile_center_map(&tile_pos, grid_size, map_type, map_transform);

                        // TileClickEvent
                        ev_tile_click.send(TileClickEvent {
                            tile_entity,
                            tile_pos,
                            tile_center,
                            api: true,
                        });
                    }
                }
            }
        }
    }
}
