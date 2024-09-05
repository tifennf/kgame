use bevy::{
    app::{Plugin, Update},
    input::{keyboard::Key, ButtonInput},
    prelude::{
        in_state, AppExtStates, Commands, IntoSystemConfigs, KeyCode, NextState, OnExit, Query,
        Res, ResMut, Resource, State, States,
    },
};
use serde::Serialize;

use crate::{
    dot::{Dot, DotColor, DotStorage},
    KAING_VALUE, TILEMAP_SIZE,
};

// Game internal state
#[derive(Resource, Clone, Serialize)]
pub struct Game {
    pub dot_color: DotColor,
    pub dot_storage: DotStorage,
    pub open: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            dot_color: DotColor::BLUE,
            dot_storage: DotStorage::empty(TILEMAP_SIZE as usize),
            open: true,
        }
    }
}

impl Game {
    // change to next player is game is not over by changing current color
    pub fn next_player(&mut self) {
        // TODO: use thread to speed up because each procedure are not dependant of each other
        let is_winner = check_rows(&self.dot_storage, &self.dot_color)
            || check_cols(&self.dot_storage, &self.dot_color)
            || check_anti_diags(&self.dot_storage, &self.dot_color)
            || check_diags(&self.dot_storage, &self.dot_color);

        if is_winner {
            self.open = false;
            self.print_state();
            return;
        }

        self.dot_color = match self.dot_color {
            DotColor::RED => DotColor::BLUE,
            DotColor::BLUE => DotColor::RED,
        };
    }

    pub fn get_turn(&self) -> u32 {
        let nb_dot = self.dot_storage.dot_count;

        (nb_dot + 1) / 2
    }
    pub fn print_state(&self) {
        if !self.open {
            println!(
                "Etat: fini\nNombre de tours: {}\nGagnant: {}",
                self.get_turn(),
                self.dot_color
            );
        } else {
            println!("Etat: en cours\nNombre de tours: {}", self.get_turn());
        }
    }
}

pub fn is_winner(game: &Game) -> bool {
    // TODO: use thread to speed up because each procedure are not dependant of each other
    let is_winner = check_rows(&game.dot_storage, &game.dot_color)
        || check_cols(&game.dot_storage, &game.dot_color)
        || check_anti_diags(&game.dot_storage, &game.dot_color)
        || check_diags(&game.dot_storage, &game.dot_color);

    is_winner
}

fn check_rows(board: &DotStorage, color: &DotColor) -> bool {
    // check for a Kaing in every rows
    // pre: p > 0

    let n = board.n;

    for j in 0..n {
        let mut count = 0;

        for i in 0..n {
            if let Some(dot) = board.get(i, j) {
                if color.to_string() == dot.color.to_string() {
                    count += 1;
                } else {
                    count = 0;
                }
            } else {
                count = 0;
            }

            if count == KAING_VALUE {
                return true;
            }
        }
    }

    false
}

fn check_cols(board: &DotStorage, color: &DotColor) -> bool {
    // check for a for a Kaing in every cols
    // pre: p > 0

    let n = board.n;

    for i in 0..n {
        let mut count = 0;

        for j in 0..n {
            if let Some(dot) = board.get(i, j) {
                if color.to_string() == dot.color.to_string() {
                    count += 1;
                } else {
                    count = 0;
                }
            } else {
                count = 0;
            }

            if count == KAING_VALUE {
                return true;
            }
        }
    }

    false
}

fn check_anti_diags(board: &DotStorage, color: &DotColor) -> bool {
    // check for a Kaing in every anti-diagonals
    // pre: p > 0

    let n = board.n;

    let k_sup = 1 + 2 * (n - 1);

    for k in 0..k_sup {
        let mut count = 0;

        let (mut i, mut j);

        if k < n {
            i = n - k - 1;
            j = 0;
        } else {
            i = 0;
            j = k % n + 1;
        }

        while i < n && j < n {
            if let Some(dot) = board.get(i, j) {
                if color.to_string() == dot.color.to_string() {
                    count += 1;
                } else {
                    count = 0;
                }
            } else {
                count = 0;
            }

            if count == KAING_VALUE {
                return true;
            }

            i += 1;
            j += 1;
        }
    }

    false
}

fn check_diags(board: &DotStorage, color: &DotColor) -> bool {
    // check a for a Kaing in every anti-diagonales
    // pre: p > 0 && board is a valid board with size >= KAING_VALUE

    let n = board.n;

    let mut i_basis = 4;
    let mut j_basis = 0;

    // check the anti-diagonales, no need for the last KAING_VALUE columns or first KAING_VALUE rows
    while i_basis < n && j_basis < n - KAING_VALUE as usize {
        let mut count = 0;

        // initial point of the current anti-diagonale
        let mut i = i_basis;
        let mut j = j_basis;

        // iterate over the anti-diagonale to check for a Kaing
        while j < n {
            if let Some(dot) = board.get(i, j) {
                if color.to_string() == dot.color.to_string() {
                    count += 1;
                } else {
                    count = 0;
                }
            } else {
                count = 0;
            }

            if count == KAING_VALUE {
                return true;
            }

            i = if i == 0 {
                break;
            } else {
                i - 1
            };
            j += 1;
        }

        // if we are not at the last anti-diagonale, we move to the next one
        if i_basis < n - 1 {
            i_basis += 1;
        } else {
            j_basis += 1;
        }
    }

    false
}

pub fn toggle_game(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::GameOver),
            GameState::GameOver => next_state.set(GameState::InGame),
        }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    InGame,
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::InGame
    }
}

// remove all dot entities and set new game data
fn clean_game(mut game: ResMut<Game>, mut commands: Commands) {
    for dot in game.dot_storage.get_storage() {
        if let Some(dot) = dot {
            commands.entity(dot.entity).despawn();
        }
    }

    *game = Game::default();
}

// print game result on state GameOver
fn game_result() {}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Game>()
            .init_state::<GameState>()
            .add_systems(OnExit(GameState::GameOver), clean_game)
            .add_systems(Update, toggle_game);
    }
}
