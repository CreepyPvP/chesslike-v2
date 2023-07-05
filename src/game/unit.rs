use std::collections::HashMap;

use bevy::{prelude::*, utils::petgraph::algo::has_path_connecting};

use crate::{assets::types::TiledMap, game::map::MapLayout, game_config::GameAssets, AppState};

use super::{
    animation::{Animatable, Animation},
    isometric::iso_transform,
    GameSystemSets,
};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitRegistry::default());
        app.add_systems((
            create_units.in_schedule(OnEnter(AppState::Game)),
            update_unit_transform.in_set(GameSystemSets::Logic),
            move_units
                .in_set(GameSystemSets::Logic)
                .before(update_unit_transform),
        ));
    }
}

#[derive(Resource, Default)]
pub struct UnitRegistry {
    pub units: HashMap<(i32, i32), Entity>,
}

#[derive(Component)]
pub struct Unit {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub travel_distance: u32,

    // waypoint index, waypoint progress, waypoints
    pub path: Option<(u32, f32, Vec<(i32, i32)>)>,
}

impl Unit {
    pub fn move_path(&mut self, path: Vec<(i32, i32)>) {
        self.path = Some((0, 0., path));
    }
}

fn create_units(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut unit_registry: ResMut<UnitRegistry>,
) {
    // this is incredibly ugly...
    let ogre_walk = Animation::new(0.4, 192, 192, 64, 64, vec![(0, 5), (1, 5), (2, 5), (3, 5)]);

    let ogre = commands
        .spawn((
            SpriteBundle {
                texture: game_assets.units.get("ogre").unwrap().clone(),
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.)),
                ..default()
            },
            Unit {
                travel_distance: 3,
                x: 1.,
                y: 1.,
                z: 1.,
                path: None,
            },
            Animatable::from_anim(ogre_walk, true),
        ))
        .id();
    unit_registry.units.insert((1, 1), ogre);
}

fn update_unit_transform(
    tilemaps: Res<Assets<TiledMap>>,
    game_assets: Res<GameAssets>,
    mut units: Query<(&mut Transform, &Unit)>,
) {
    let tilemap = tilemaps.get(&game_assets.map).unwrap();
    let (tile_w, tile_h) = (tilemap.tilewidth as f32, tilemap.tileheight as f32);

    for (mut transform, unit) in units.iter_mut() {
        transform.translation = iso_transform(unit.x, unit.y, unit.z, tile_w, tile_h, true);
    }
}

fn move_units(
    mut units: Query<(&mut Unit, Entity)>,
    time: Res<Time>,
    map_layout: Res<MapLayout>,
    mut unit_registry: ResMut<UnitRegistry>,
) {
    for (mut unit, entity) in units.iter_mut() {
        if unit.path.is_none() {
            continue;
        }
        let path = std::mem::replace(&mut unit.path, None);
        let (mut current_waypoint, mut progress, path) = path.unwrap();

        progress += 0.5 / time.delta().as_millis() as f32;

        if progress > 1.0 {
            progress = 0.;
            current_waypoint += 1;
            if current_waypoint as usize == path.len() - 1 {
                let last_waypoint = path.last().unwrap();
                unit.x = last_waypoint.0 as f32;
                unit.y = last_waypoint.1 as f32;
                unit_registry.units.remove(&path[0]);
                unit_registry.units.insert(*last_waypoint, entity);
                continue;
            }
        }

        unit.x = (1. - progress) * path[current_waypoint as usize].0 as f32
            + progress * path[current_waypoint as usize + 1].0 as f32;
        unit.y = (1. - progress) * path[current_waypoint as usize].1 as f32
            + progress * path[current_waypoint as usize + 1].1 as f32;

        unit.path = Some((current_waypoint, progress, path));
    }
}
