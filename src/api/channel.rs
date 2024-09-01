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

pub fn setup_bevy_channel(
    chan: Res<ChannelManager<BevyMessage, ServerMessage>>,
    gstate: Res<GameState>,
) {
    while let Ok(msg) = chan.rx.try_recv() {
        let s = gstate.as_ref().clone();
        match msg {
            ServerMessage::GetState => {
                if let Err(e) = chan.tx.try_send(BevyMessage::GameState(s)) {
                    println!("{}", e);
                }
            }
            ServerMessage::PlaceDot => todo!(),
        }
    }
}
