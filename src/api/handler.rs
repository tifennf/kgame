use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use bevy_ecs_tilemap::tiles::TilePos;
use serde::Deserialize;

use crate::{dot::DotColor, game::Game};

use super::channel::{BevyMessage, ChannelManager, ServerMessage};

type Chan = Arc<ChannelManager<ServerMessage, BevyMessage>>;

#[derive(Deserialize, Clone)]
pub struct ApiDot {
    pub color: DotColor,
    pub tilemap_pos: (u32, u32),
}

impl ApiDot {
    pub fn get_tilepos(&self) -> TilePos {
        let (x, y) = self.tilemap_pos;

        TilePos { x, y }
    }
}

// sendback game state on GET /state
pub async fn get_game_data(State(chan): State<Chan>) -> Json<Option<Game>> {
    chan.tx.send_async(ServerMessage::GetState).await.unwrap();

    if let BevyMessage::State(d) = chan.rx.recv_async().await.unwrap() {
        Json(Some(d))
    } else {
        Json(None)
    }
}

// POST /dot
pub async fn place_dot(State(chan): State<Chan>, Json(payload): Json<ApiDot>) -> StatusCode {
    chan.tx
        .send_async(ServerMessage::PlaceDot(payload))
        .await
        .unwrap();

    if let BevyMessage::DotPlaced = chan.rx.recv_async().await.unwrap() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}
