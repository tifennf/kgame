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

use crate::{game_state::GameState, utils::CursorPos};
#[derive(Component)]
pub struct Dot {
    pub entity: Entity,
    pub color: DotColor,
    pub pos: TilePos,
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

// dot storage implemented as a stack
pub struct DotStorage {
    pub max_len: u32,
    stack: Vec<Dot>,
}

impl DotStorage {
    // create a new empty DotStorage
    pub fn empty(map_size: u32) -> DotStorage {
        let max_len = map_size * map_size;

        Self {
            max_len,
            stack: Vec::new(),
        }
    }

    // add dot in the stack, panic if full -> todo
    pub fn push(&mut self, dot: Dot) {
        let nb_elem: u32 = self.stack.len() as u32;

        if nb_elem == self.max_len {
            panic!("too many dot");
        }

        self.stack.push(dot);
    }

    // remove last inserted dot and return it
    pub fn pop(&mut self) -> Option<Dot> {
        self.stack.pop()
    }

    // return a ref to the dot located at pos
    pub fn peek(&self, pos: &TilePos) -> Option<&Dot> {
        let res = None;

        for dot in self.stack.iter() {
            if dot.pos == *pos {
                return Some(dot);
            }
        }

        res
    }
}

// system function to spawn dot when player click on a tile, according to his color
pub fn spawn_dot_on_click(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    buttons: Res<ButtonInput<MouseButton>>,
    q_tilemap: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gstate: ResMut<GameState>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (map_size, grid_size, map_type, tile_storage, map_transform) in q_tilemap.iter() {
            // needed in order to match tile with cursor
            let position = cursor_pos.0;
            let cursor_in_map_pos: Vec2 = {
                // Extend the cursor_pos vec3 by 0.0 and 1.0
                let cursor_pos = Vec4::from((position, 0.0, 1.0));
                let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
                cursor_in_map_pos.xy()
            };

            if let Some(tile_pos) =
                TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
            {
                // compute tile center, 2 spaces involved: world, tilemap
                let center = {
                    // tile center in world basis
                    let center_world = tile_pos.center_in_world(grid_size, map_type);

                    // using tilemap transformation, we change tile center basis to tilemap basis
                    let center_tilemap =
                        map_transform.compute_matrix() * Vec4::from((center_world, 0.0, 1.0));

                    center_tilemap.xy()
                };

                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    match gstate.dot_storage.peek(&tile_pos) {
                        Some(_) => (),
                        None => {
                            // spawn dot entity
                            let dot_entity = commands
                                .spawn(MaterialMesh2dBundle {
                                    mesh: meshes.add(Circle::default()).into(),
                                    transform: Transform::default()
                                        .with_scale(Vec3::splat(16.0))
                                        .with_translation(center.xy().extend(1.0)), // here
                                    material: materials.add(Color::from(gstate.dot_color.clone())),
                                    ..default()
                                })
                                .id();

                            // create a new dot component
                            let new_dot = Dot {
                                entity: dot_entity,
                                pos: tile_pos,
                                color: gstate.dot_color.clone(),
                            };

                            // place dot on top of target tile and register it into dot storage
                            commands.entity(tile_entity).insert(new_dot.clone());
                            gstate.dot_storage.push(new_dot);

                            gstate.next_turn();
                        }
                    }
                }
            }
        }
    }
}
