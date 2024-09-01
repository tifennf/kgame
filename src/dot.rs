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
use serde::{ser::SerializeStruct, Serialize};

use crate::{game::GameState, utils::CursorPos};

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

#[derive(Serialize)]
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
        if !gstate.open {
            gstate.print_state();
            return;
        }

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

                            gstate.change_color();
                        }
                    }
                }
            }
        }
    }
}
