use std::collections::HashMap;

use bevy::{prelude::*, render::color::SrgbColorSpace};

use crate::{
    assets::types::TiledMap, game_config::GameAssets, util::collisions::Triangle, AppState,
};

use super::{
    isometric::iso_transform,
    picking::{PickState, Pickable},
    unit::UnitRegistry,
    GameSystemSets,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapState::default());
        app.insert_resource(MapLayout::default());

        app.add_systems((
            create_map.in_schedule(OnEnter(AppState::Game)),
            destroy_map.in_schedule(OnExit(AppState::Game)),
            update_tile_selection.in_set(GameSystemSets::Logic),
            update_tint.in_set(GameSystemSets::Render),
        ));
    }
}

#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Tile {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Tile { x, y, z }
    }
}

#[derive(Component)]
pub struct Map;

#[derive(Resource, Default)]
pub struct MapLayout {
    pub tiles: HashMap<(i32, i32), u32>,
}

#[derive(Resource, Default)]
pub struct MapState {
    tile_tints: HashMap<(i32, i32), Color>,
}

pub fn create_map(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    tilemaps: Res<Assets<TiledMap>>,
    mut map_layout: ResMut<MapLayout>,
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
    for (layer_id, layer) in tilemap.layers.iter().enumerate() {
        let layer_id = layer_id as u32;
        let layer_id_f = layer_id as f32;

        for editor_x in 0..layer.width {
            for editor_y in 0..layer.height {
                let id = layer.data[(editor_x + editor_y * layer.width) as usize] as usize;
                if id == 0 {
                    continue;
                }

                let (x, y) = correct_editor_transform(editor_x, editor_y, layer_id);
                map_layout.tiles.insert((x, y), layer_id);
                let tile = commands
                    .spawn((
                        SpriteBundle {
                            texture: game_assets.tiles[id - 1].clone(),
                            transform: iso_transform(
                                x as f32, y as f32, layer_id_f, tile_w, tile_h, false,
                            ),
                            ..default()
                        },
                        pickable.clone(),
                        Tile::new(x, y, layer_id as i32),
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

fn update_tile_selection(
    tiles: Query<(&Tile, Entity)>,
    pick_state: Res<PickState>,
    mouse: Res<Input<MouseButton>>,
    unit_registry: Res<UnitRegistry>,
    map_layout: Res<MapLayout>,
    mut map_state: ResMut<MapState>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(entity) = pick_state.selected {
        for (tile, tile_entity) in &tiles {
            if tile_entity != entity {
                continue;
            }
            map_state.tile_tints.clear();
            let unit = unit_registry.units.get(&(tile.x, tile.y));
            if unit.is_some() {
                let paths = find_unit_paths(2, (tile.x, tile.y), &map_layout);
                for reachable_tile in paths.keys() {
                    let (x, y) = *reachable_tile;
                    map_state.tile_tints.insert((x, y), Color::rgb(0.6, 1.0, 0.6));
                }
            }
            return;
        }
    }
}

fn update_tint(
    pick_state: Res<PickState>,
    mut tiles: Query<(&mut Sprite, &Tile, Entity)>,
    map_state: Res<MapState>,
) {
    for (mut sprite, tile, entity) in tiles.iter_mut() {

        let mut color = map_state
            .tile_tints
            .get(&(tile.x, tile.y))
            .map(|color| color.clone())
            .unwrap_or(Color::WHITE);

        if let Some(selected) = pick_state.selected {
            if selected == entity {
               color = Color::rgb(1.2 * color.r(), 1.2 * color.g(), 1.2 * color.b());
            }
        }

        sprite.color = color;
    }
}

pub fn destroy_map(mut command: Commands, query: Query<Entity, With<Map>>) {
    if let Ok(map_entity) = query.get_single() {
        command.entity(map_entity).despawn_recursive();
    }
}

//
// Util

fn correct_editor_transform(editor_x: u32, editor_y: u32, layer_id: u32) -> (i32, i32) {
    (
        editor_x as i32 - 1 + layer_id as i32,
        editor_y as i32 - 1 + layer_id as i32,
    )
}

pub fn find_unit_paths(distance: u32, location: (i32, i32), map_layout: &Res<MapLayout>) -> HashMap<(i32, i32), (i32, i32)> {
    let mut paths = HashMap::new();
    paths.insert(location.clone(), location.clone());
    let mut prev = vec!(location);
    for _ in 0..distance {
        let mut next_prev: Vec<(i32, i32)> = vec!();
        for prev_tile in &prev {
            let (x, y) = *prev_tile;

            let dest = (x - 1, y);
            if !paths.contains_key(&dest) && can_go_from_to(prev_tile, &dest, map_layout) {
                paths.insert(dest, prev_tile.clone());
                next_prev.push(dest);
            }

            let dest = (x + 1, y);
            if !paths.contains_key(&dest) && can_go_from_to(prev_tile, &dest, map_layout) {
                paths.insert(dest, prev_tile.clone());
                next_prev.push(dest);
            }

            let dest = (x, y - 1);
            if !paths.contains_key(&dest) && can_go_from_to(prev_tile, &dest, map_layout) {
                paths.insert(dest, prev_tile.clone());
                next_prev.push(dest);
            }

            let dest = (x, y + 1);
            if !paths.contains_key(&dest) && can_go_from_to(prev_tile, &dest, map_layout) {
                paths.insert(dest, prev_tile.clone());
                next_prev.push(dest);
            }
        }

        prev = next_prev;
    }

    paths
}

fn can_go_from_to(from: &(i32, i32), to: &(i32, i32), map_layout: &Res<MapLayout>) -> bool {
    map_layout.tiles.get(from) == map_layout.tiles.get(to)
}
