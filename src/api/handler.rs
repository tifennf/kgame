use std::sync::Arc;

use axum::extract::State;

use crate::game::GameState;

use super::channel::{BevyMessage, ChannelManager, ServerMessage};

type Chan = Arc<ChannelManager<ServerMessage, BevyMessage>>;

pub async fn get_game_state(State(state): State<Chan>) -> String {
    state.tx.send_async(ServerMessage::GetState).await.unwrap();

    println!("prout");

    // something wrong there
    let BevyMessage::GameState(s): BevyMessage = state.rx.recv_async().await.unwrap();

    s.get_turn().to_string()
}
