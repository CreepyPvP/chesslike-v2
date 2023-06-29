use crate::{assets::types::{TiledMap, TiledSet}, types::GameAssets, AppState};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum GameSystemSets {
    Start,
    Input,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_set(GameSystemSets::Start);
        app.configure_set(GameSystemSets::Input);

        app.add_systems((
            create_map
                .in_schedule(OnEnter(AppState::Game))
                .in_set(GameSystemSets::Start),
            destroy_map.in_schedule(OnExit(AppState::Game)),
        ));
    }
}

#[derive(Component)]
struct Map;

fn create_map(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    tilemaps: Res<Assets<TiledMap>>,
) {
    let tilemap = tilemaps.get(&game_assets.map).unwrap();
    let (tile_w, tile_h) = (tilemap.tilewidth as f32, tilemap.tileheight as f32);

    let mut tiles: Vec<Entity> = vec![];
    for (i, layer) in tilemap.layers.iter().enumerate() {
        let l_width = layer.width as f32;
        let l_height = layer.height as f32;
        let layer_id = i as f32;

        for x in 0..layer.width {
            for y in 0..layer.height {
                let id = layer.data[(x + y * layer.width) as usize] as usize;
                if id == 0 {
                    continue;
                }

                let x = x as f32 + layer_id;
                let y = y as f32 + layer_id;

                let draw_x = (x * tile_w - y * tile_w) / 2.;
                let draw_y = (x * tile_h + y * tile_h) / 2.;
                let tile = commands
                    .spawn(SpriteBundle {
                        texture: game_assets.tiles[id - 1].clone(),
                        transform: Transform::from_xyz(draw_x, -draw_y + layer_id * tile_h, x + y * l_width + layer_id),
                        ..default()
                    })
                    .id();
                tiles.push(tile);
            }
        }
    }

    let mut map = commands.spawn((Map, SpatialBundle {
        transform: Transform::from_scale(Vec3::new(0.4, 0.4, 1.)),
        ..default()
    }));
    for tile in tiles {
        map.add_child(tile);
    }
}

fn destroy_map(mut command: Commands, query: Query<Entity, With<Map>>) {
    if let Ok(map_entity) = query.get_single() {
        command.entity(map_entity).despawn_recursive();
    }
}
