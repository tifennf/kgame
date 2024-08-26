use bevy::{
    app::{Plugin, Startup},
    asset::{AssetServer, Handle},
    color::Color,
    prelude::{Camera2dBundle, ClearColor, Commands, Component, Image, Res},
};
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    prelude::get_tilemap_center_transform,
    tiles::{TileBundle, TilePos, TileStorage},
    TilemapBundle,
};

const N: u32 = 10;

// create the grid with N**2 node in it
fn setup_grid(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // camera for rendering
    cmd.spawn(Camera2dBundle::default());

    // load the asset
    let texture_handle: Handle<Image> = asset_server.load("tile.png");

    // set grid size ( unit is tile )
    let map_size = TilemapSize { x: 32, y: 32 };

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

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 }; // px
    let grid_size = tile_size.into(); // same size
    let map_type = TilemapType::default(); // square tile

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

    // #[cfg(all(not(feature = "atlas"), feature = "render"))]
    // {
    //     array_texture_loader.add(TilemapArrayTexture {
    //         texture: TilemapTexture::Single(asset_server.load("tiles.png")),
    //         tile_size,
    //         ..Default::default()
    //     });
    // }
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_grid)
            .insert_resource(ClearColor(Color::WHITE));
    }
}
