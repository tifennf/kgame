use std::sync::Arc;

use axum::{extract::State, Json};

use crate::game::GameState;

use super::channel::{BevyMessage, ChannelManager, ServerMessage};

type Chan = Arc<ChannelManager<ServerMessage, BevyMessage>>;

// sendback game state on GET /
pub async fn get_game_state(State(state): State<Chan>) -> Json<GameState> {
    state.tx.send_async(ServerMessage::GetState).await.unwrap();

    let BevyMessage::State(s): BevyMessage = state.rx.recv_async().await.unwrap();

    Json(s)
}
