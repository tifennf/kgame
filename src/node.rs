use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};

use crate::utils::CursorPos;

#[derive(Component)]
pub struct Node;

pub fn handle_click(
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
) {
    //
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
                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    println!("tile pos: ({}, {})", tile_pos.x, tile_pos.y);
                    commands.entity(tile_entity).insert(Node);
                }
            }
        }
    }
}
