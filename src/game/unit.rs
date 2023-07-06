use std::collections::HashMap;

use bevy::prelude::*;

use crate::{assets::types::TiledMap, game::map::MapLayout, game_config::GameAssets, AppState, math::max};

use super::{
    animation::{Animatable, Animation},
    isometric::{iso_transform, self},
    GameSystemSets, map::MapState,
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

    // movement
    pub travel_distance: u32,
    pub is_air: bool,

    // waypoint index, waypoint progress, waypoints
    pub path: Option<(u32, f32, Vec<(i32, i32)>)>,
    render_priority: Option<f32>,
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
                render_priority: None,
                is_air: false,
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
        if let Some(render_prio) = unit.render_priority {
            transform.translation.z = render_prio;
        }
    }
}

fn move_units(
    mut units: Query<(&mut Unit, Entity)>,
    time: Res<Time>,
    map_layout: Res<MapLayout>,
    mut unit_registry: ResMut<UnitRegistry>,
    mut map_state: ResMut<MapState>,
) {
    for (mut unit, entity) in units.iter_mut() {
        if unit.path.is_none() {
            continue;
        }
        let path = std::mem::replace(&mut unit.path, None);
        let (mut current_waypoint, mut progress, path) = path.unwrap();

        if unit.render_priority.is_none() {
            let waypoint_1 = path[current_waypoint as usize];
            let prio_1 = iso_transform(waypoint_1.0 as f32, waypoint_1.1 as f32, map_layout.tiles[&(waypoint_1.0, waypoint_1.1)] as f32, 1., 1., true).z;
            let waypoint_2 = path[current_waypoint as usize + 1];
            let prio_2 = iso_transform(waypoint_2.0 as f32, waypoint_2.1 as f32, map_layout.tiles[&(waypoint_2.0, waypoint_2.1)] as f32, 1., 1., true).z;
            let render_prio = max(prio_1, prio_2);
            unit.render_priority = Some(render_prio);
        }

        progress += 0.2 / time.delta().as_millis() as f32;

        if progress > 1.0 {
            progress = 0.;
            current_waypoint += 1;
            unit.render_priority = None;
            if current_waypoint as usize == path.len() - 1 {
                let last_waypoint = path.last().unwrap();
                unit.x = last_waypoint.0 as f32;
                unit.y = last_waypoint.1 as f32;
                unit_registry.units.remove(&path[0]);
                unit_registry.units.insert(*last_waypoint, entity);
                unit.render_priority = None;
                map_state.unit_moving = false;
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
