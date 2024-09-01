use std::future::IntoFuture;

use bevy::{
    prelude::{Res, Resource},
    tasks::IoTaskPool,
};
use flume::{Receiver, Sender};
use serde_json::Value;

use crate::game::GameState;

// from bevy to server
#[derive(Clone)]
pub enum BevyMessage {
    State(GameState),
}

// from server to bevy
#[derive(Clone)]
pub enum ServerMessage {
    GetState,
    PlaceDot,
}

#[derive(Resource, Clone)]
pub struct ChannelManager<T, L> {
    pub tx: Sender<T>,
    pub rx: Receiver<L>,
}

// communicate with server using channel in order to perform player's actions
pub fn handle_bevy_channel(
    chan: Res<ChannelManager<BevyMessage, ServerMessage>>,
    gstate: Res<GameState>,
) {
    while let Ok(msg) = chan.rx.try_recv() {
        let s = gstate.as_ref().clone();
        match msg {
            ServerMessage::GetState => {
                if let Err(e) = chan.tx.try_send(BevyMessage::State(s)) {
                    println!("{}", e);
                }
            }
            ServerMessage::PlaceDot => todo!(),
        }
    }
}
