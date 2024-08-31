use std::future::IntoFuture;

use bevy::prelude::{Res, Resource};
use flume::{Receiver, Sender};
use serde_json::Value;

use crate::game::GameState;

// from bevy to server
#[derive(Clone)]
pub enum BevyMessage {
    GameState(GameState),
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

fn setup_bevy_channel(
    chan: Res<ChannelManager<BevyMessage, ServerMessage>>,
    gstate: Res<GameState>,
) {
    let s = gstate.as_ref();

    match chan.rx.recv().unwrap() {
        ServerMessage::GetState => {
            chan.tx.send(BevyMessage::GameState(s.clone())).unwrap();
        }
        ServerMessage::PlaceDot => todo!(),
    }
}
