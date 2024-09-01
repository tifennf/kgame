use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Handle},
    color::Color,
    input::ButtonInput,
    math::{Vec2, Vec4, Vec4Swizzles},
    prelude::{
        Camera2dBundle, ClearColor, Commands, Entity, Event, EventWriter, Image, MouseButton,
        Query, Res, Transform,
    },
};
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    prelude::get_tilemap_center_transform,
    tiles::{TileBundle, TilePos, TileStorage},
    TilemapBundle,
};

use crate::{utils::CursorPos, TILEMAP_SIZE, TILE_ASSET_PATH, TILE_ASSET_SIZE};

fn setup_grid(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // camera for rendering
    cmd.spawn(Camera2dBundle::default());

    // load the asset
    let texture_handle: Handle<Image> = asset_server.load(TILE_ASSET_PATH);

    // set grid size ( unit is tile )
    let map_size = TilemapSize {
        x: TILEMAP_SIZE,
        y: TILEMAP_SIZE,
    };

    // create a entity for the grid and a collection of tiles
    let tilemap_entity = cmd.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    // place tiles on the map
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = cmd
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();

            // register tile
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize {
        x: TILE_ASSET_SIZE,
        y: TILE_ASSET_SIZE,
    }; // px
    let grid_size = tile_size.into(); // same size
    let map_type = TilemapType::Square; // square tile

    cmd.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

// system function to spawn dot when player click on a tile, according to his color
pub fn handle_tile_click(
    cursor_pos: Res<CursorPos>,
    buttons: Res<ButtonInput<MouseButton>>,
    q_tilemap: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,

    mut ev_tile_click: EventWriter<TileClickEvent>,
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
                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    // compute tile center, 2 spaces involved: world, tilemap
                    let tile_center = {
                        // tile center in world basis
                        let center_world = tile_pos.center_in_world(grid_size, map_type);

                        // using tilemap transformation, we change tile center basis to tilemap basis
                        let center_tilemap =
                            map_transform.compute_matrix() * Vec4::from((center_world, 0.0, 1.0));

                        center_tilemap.xy()
                    };

                    // TileClickEvent
                    ev_tile_click.send(TileClickEvent {
                        tile_entity,
                        tile_pos,
                        tile_center,
                        api: false,
                    });
                }
            }
        }
    }
}

#[derive(Event, Clone)]
pub struct TileClickEvent {
    pub tile_entity: Entity,
    pub tile_pos: TilePos,
    pub tile_center: Vec2,
    pub api: bool,
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<TileClickEvent>()
            .add_systems(Startup, setup_grid)
            .add_systems(Update, handle_tile_click)
            .insert_resource(ClearColor(Color::WHITE));
    }
}
