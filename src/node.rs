use bevy::{
    color::palettes::css::RED,
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};

use crate::utils::CursorPos;

#[derive(Component)]
pub struct Node(Entity);

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                    // compute tile center, 2 spaces involved: world, tilemap
                    let center = {
                        // tile center in world basis
                        let center_world = tile_pos.center_in_world(grid_size, map_type);

                        // using tilemap transformation, we change tile center basis to tilemap basis
                        let center_tilemap =
                            map_transform.compute_matrix() * Vec4::from((center_world, 0.0, 1.0));

                        center_tilemap.xy()
                    };

                    let node_entity = commands
                        .spawn(MaterialMesh2dBundle {
                            mesh: meshes.add(Circle::default()).into(),
                            transform: Transform::default()
                                .with_scale(Vec3::splat(16.0))
                                .with_translation(center.xy().extend(1.0)), // here
                            material: materials.add(Color::from(RED)),
                            ..default()
                        })
                        .id();

                    println!("({}, {})", center.x, center.y);

                    commands.entity(tile_entity).insert(Node(node_entity));
                }
            }
        }
    }
}
