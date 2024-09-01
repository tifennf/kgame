use bevy::{app::Plugin, prelude::Resource};
use serde::Serialize;

use crate::{
    dot::{DotColor, DotStorage},
    KAING_VALUE, TILEMAP_SIZE,
};

// Game internal state
#[derive(Resource, Clone, Serialize)]
pub struct GameState {
    pub dot_color: DotColor,
    pub dot_storage: DotStorage,
    pub open: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            dot_color: DotColor::BLUE,
            dot_storage: DotStorage::empty(TILEMAP_SIZE as usize),
            open: true,
        }
    }
}

impl GameState {
    pub fn change_color(&mut self) {
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

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<GameState>();
    }
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
