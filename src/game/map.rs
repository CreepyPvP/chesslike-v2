use bevy::prelude::*;

use crate::{
    assets::types::TiledMap, game_config::GameAssets, util::collisions::Triangle, AppState,
};

use super::{hoverable::Hoverable, isometric::iso_transform, picking::Pickable};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            create_map.in_schedule(OnEnter(AppState::Game)),
            destroy_map.in_schedule(OnExit(AppState::Game)),
        ));
    }
}

#[derive(Component)]
pub struct Map;

pub fn create_map(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    tilemaps: Res<Assets<TiledMap>>,
) {
    let tilemap = tilemaps.get(&game_assets.map).unwrap();
    let (tile_w, tile_h) = (tilemap.tilewidth as f32, tilemap.tileheight as f32);

    let pickable = Pickable {
        triangles: vec![
            Triangle::new(
                Vec2::new(116., 0.),
                Vec2::new(-116., 0.),
                Vec2::new(0., 45.),
            ),
            Triangle::new(
                Vec2::new(116., 0.),
                Vec2::new(-116., 0.),
                Vec2::new(-116., -116.),
            ),
            Triangle::new(
                Vec2::new(-116., -116.),
                Vec2::new(116., -116.),
                Vec2::new(116., 0.),
            ),
            Triangle::new(
                Vec2::new(116., -116.),
                Vec2::new(-116., -116.),
                Vec2::new(0., -170.),
            ),
        ],
    };

    let mut tiles: Vec<Entity> = vec![];
    for (i, layer) in tilemap.layers.iter().enumerate() {
        let layer_id = i as f32;

        for editor_x in 0..layer.width {
            for editor_y in 0..layer.height {
                let id = layer.data[(editor_x + editor_y * layer.width) as usize] as usize;
                if id == 0 {
                    continue;
                }

                let (x, y) = correct_editor_transform(editor_x, editor_y, layer_id);

                let tile = commands
                    .spawn((
                        SpriteBundle {
                            texture: game_assets.tiles[id - 1].clone(),
                            transform: iso_transform(x, y, layer_id, tile_w, tile_h),
                            ..default()
                        },
                        pickable.clone(),
                        Hoverable,
                    ))
                    .id();
                tiles.push(tile);
            }
        }
    }

    let mut map = commands.spawn((
        Map,
        SpatialBundle {
            // transform: Transform::from_scale(Vec3::new(0.4, 0.4, 1.)),
            ..default()
        },
    ));
    for tile in tiles {
        map.add_child(tile);
    }
}

fn correct_editor_transform(editor_x: u32, editor_y: u32, layer_id: f32) -> (f32, f32) {
    (
        editor_x as f32 - 1.0 + layer_id,
        editor_y as f32 - 1.0 + layer_id,
    )
}

pub fn destroy_map(mut command: Commands, query: Query<Entity, With<Map>>) {
    if let Ok(map_entity) = query.get_single() {
        command.entity(map_entity).despawn_recursive();
    }
}
