use core::fmt;

use bevy::{
    color::palettes::css::{BLUE, RED},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use crate::{
    api::channel::{BevyMessage, ChannelManager, ServerMessage},
    game::{is_winner, Game, GameState},
    grid::TileClickEvent,
    utils::CursorPos,
};

// each player can place a dot on grid
#[derive(Component)]
pub struct Dot {
    pub entity: Entity,
    pub color: DotColor,
    pub pos: TilePos,
}

impl Serialize for Dot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Dot", 3)?;
        state.serialize_field("entity", &self.entity)?;
        state.serialize_field("color", &self.color)?;
        state.serialize_field("pos", &[self.pos.x, self.pos.y])?;
        state.end()
    }
}

impl Clone for Dot {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            color: self.color.clone(),
            pos: self.pos.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum DotColor {
    RED,
    BLUE,
}

impl From<DotColor> for Color {
    fn from(val: DotColor) -> Self {
        match val {
            DotColor::RED => Color::from(RED),
            DotColor::BLUE => Color::from(BLUE),
        }
    }
}

impl Clone for DotColor {
    fn clone(&self) -> Self {
        match self {
            Self::RED => Self::RED,
            Self::BLUE => Self::BLUE,
        }
    }
}

impl fmt::Display for DotColor {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DotColor::RED => fmt.write_str("RED")?,
            DotColor::BLUE => fmt.write_str("BLUE")?,
        }

        Ok(())
    }
}

// dot storage implemented as a flat matrix
#[derive(Clone, Serialize)]
pub struct DotStorage {
    pub dot_count: u32,
    pub n: usize,
    matrix: Vec<Option<Dot>>,
}

impl DotStorage {
    // create a new empty DotStorage
    pub fn empty(map_size: usize) -> DotStorage {
        let matrix = vec![None; map_size * map_size];

        Self {
            dot_count: 0,
            matrix,
            n: map_size,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Option<Dot> {
        &self.matrix[x * self.n + y]
    }
    // add dot in the stack, panic if full -> todo
    pub fn push(&mut self, dot: Dot) {
        let nb_dot_max = self.matrix.len() as u32;

        if self.dot_count == nb_dot_max {
            panic!("too many dot");
        }

        let (x, y) = (dot.pos.x as usize, dot.pos.y as usize);

        self.matrix[x * self.n + y] = Some(dot);
        self.dot_count += 1;
    }

    // return a ref to the dot located at pos
    pub fn peek(&self, pos: &TilePos) -> &Option<Dot> {
        let (x, y) = (pos.x as usize, pos.y as usize);

        &self.matrix[x * self.n + y]
    }
}

// place a dot on the grid when a TileClickEvent occur
pub fn spawn_dot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game: ResMut<Game>,
    mut ev_tile_click: EventReader<TileClickEvent>,
    chan: Res<ChannelManager<BevyMessage, ServerMessage>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for ev in ev_tile_click.read().cloned() {
        if let GameState::GameOver = state.get() {
            game.print_state();
            return;
        }

        let TileClickEvent {
            tile_entity,
            tile_pos,
            tile_center,
            api,
        } = ev;

        match game.dot_storage.peek(&tile_pos) {
            Some(_) => {
                if api {
                    chan.tx.try_send(BevyMessage::InvalidDotPosition).unwrap();
                }
            }
            None => {
                // spawn dot entity
                let dot_entity = commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::default()).into(),
                        transform: Transform::default()
                            .with_scale(Vec3::splat(16.0))
                            .with_translation(tile_center.xy().extend(1.0)), // here
                        material: materials.add(Color::from(game.dot_color.clone())),
                        ..default()
                    })
                    .id();

                // create a new dot component
                let new_dot = Dot {
                    entity: dot_entity,
                    pos: tile_pos,
                    color: game.dot_color.clone(),
                };

                // place dot on top of target tile and register it into dot storage
                commands.entity(tile_entity).insert(new_dot.clone());
                game.dot_storage.push(new_dot);
                if api {
                    chan.tx.try_send(BevyMessage::DotPlaced).unwrap();
                }

                if is_winner(&game) {
                    next_state.set(GameState::GameOver);
                    game.print_state();
                    return;
                }

                game.next_player();
            }
        }
    }
}
