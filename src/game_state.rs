use bevy::{
    app::Plugin,
    color::{
        palettes::css::{BLUE, RED},
        Color,
    },
    prelude::Resource,
};

#[derive(Resource)]
pub struct GameState {
    pub turn: u32,
    pub node_color: Color,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            turn: 1,
            node_color: Color::from(BLUE),
        }
    }
}

impl GameState {
    pub fn next_turn(&mut self) {
        let current_color = self.node_color;

        if current_color == Color::from(BLUE) {
            self.node_color = Color::from(RED);
        } else {
            self.node_color = Color::from(BLUE);
        }

        self.turn += 1;
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<GameState>();
    }
}
