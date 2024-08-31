use axum::Json;
use bevy::prelude::Resource;
use flume::{Receiver, Sender};
use serde_json::Value;

// from bevy to server
#[derive(Clone)]
pub enum BevyMessage {}

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
