use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    assets::types::TiledMap, game::map::MapLayout, game_config::GameAssets, math::max, AppState,
};

use super::{
    animation::{Animatable, Animation},
    game_state::GameStateEvent,
    isometric::{iso_transform, IsometricDirection},
    map::MapState,
    GameEvent, GameSystemSets,
};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitRegistry::default());
        app.add_systems((
            process_unit_event.in_set(GameSystemSets::Update),
            apply_system_buffers.after(process_unit_event),
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

    // movement stats
    pub travel_distance: u32,
    travel_speed: f32,
    pub is_air: bool,

    // animations
    idle: Animation,
    move_up_right: Animation,
    move_up_left: Animation,
    move_down_right: Animation,
    move_down_left: Animation,

    // movement state
    // waypoint index, waypoint progress, waypoints
    path: Option<(u32, Vec<(i32, i32)>)>,
    path_progress: Option<f32>,
    render_priority: Option<f32>,
}

impl Unit {
    pub fn move_path(&mut self, path: Vec<(i32, i32)>) {
        self.path = Some((0, path));
    }
}

fn generate_entity(
    x: i32,
    y: i32,
    z: i32,
    commands: &mut Commands,
    texture: Handle<Image>,
    idle: Animation,
    move_up_left: Animation,
    move_up_right: Animation,
    move_down_left: Animation,
    move_down_right: Animation,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                texture,
                sprite: Sprite {
                    rect: Some(Rect::new(0., 0., 0., 0.)),
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 0.5)),
                ..default()
            },
            Unit {
                travel_distance: 3,
                x: x as f32,
                y: y as f32,
                z: z as f32,
                path: None,
                path_progress: None,
                render_priority: None,
                is_air: false,
                travel_speed: 0.25,
                idle: idle.clone(),
                move_up_left,
                move_up_right,
                move_down_left,
                move_down_right,
            },
            Animatable::from_anim(idle),
        ))
        .id()
}

fn process_unit_event(
    mut event_reader: EventReader<GameEvent>,
    mut event_writer: EventWriter<GameStateEvent>,
    mut unit_registry: ResMut<UnitRegistry>,
    map_layout: Res<MapLayout>,
    game_assets: Res<GameAssets>,
    mut commands: Commands,
) {
    for event in event_reader.iter() {
        match event {
            GameEvent::SpawnUnit(x, y) => {
                place_ogre(*x, *y, &game_assets, &map_layout, &mut commands, &mut unit_registry, &mut event_writer);
            }
            GameEvent::PlaceAiUnit => {
                let (mut x, mut y) = (-1, -1);
                loop {
                    x += 1;
                    y += 1;
                    if map_layout.tiles.contains_key(&(x, y)) && !unit_registry.units.contains_key(&(x, y)) {
                        break;
                    }
                }
                place_ogre(x, y, &game_assets, &map_layout, &mut commands, &mut unit_registry, &mut event_writer);
            }
        }
    }
}

fn place_ogre(
    x: i32,
    y: i32,
    game_assets: &Res<GameAssets>,
    map_layout: &Res<MapLayout>,
    commands: &mut Commands,
    unit_registry: &mut ResMut<UnitRegistry>,
    event_writer: &mut EventWriter<GameStateEvent>,
) {
    let texture = game_assets.units.get("ogre").unwrap().clone();
    let z = *map_layout.tiles.get(&(x, y)).unwrap() as i32;
    let ogre_idle = Animation::new(0.4, 192, 192, 64, 64, vec![(0, 5)], true);
    let ogre_walk_down_right = Animation::new(
        0.4,
        192,
        192,
        64,
        64,
        vec![(0, 5), (1, 5), (2, 5), (3, 5)],
        true,
    );
    let ogre_walk_down_left = Animation::new(
        0.4,
        192,
        192,
        64,
        64,
        vec![(0, 7), (1, 7), (2, 7), (3, 7)],
        true,
    );
    let ogre_walk_up_left = Animation::new(
        0.4,
        192,
        192,
        64,
        64,
        vec![(0, 1), (1, 1), (2, 1), (3, 1)],
        true,
    );
    let ogre_walk_up_right = Animation::new(
        0.4,
        192,
        192,
        64,
        64,
        vec![(0, 3), (1, 3), (2, 3), (3, 3)],
        true,
    );
    let entity = generate_entity(
        x,
        y,
        z,
        commands,
        texture,
        ogre_idle,
        ogre_walk_up_left,
        ogre_walk_up_right,
        ogre_walk_down_left,
        ogre_walk_down_right,
    );

    unit_registry.units.insert((x, y), entity);
    event_writer.send(GameStateEvent::SpawnedUnit(entity));
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
    mut units: Query<(&mut Unit, &mut Animatable, Entity)>,
    time: Res<Time>,
    map_layout: Res<MapLayout>,
    mut unit_registry: ResMut<UnitRegistry>,
    mut map_state: ResMut<MapState>,
) {
    for (mut unit, mut animatable, entity) in units.iter_mut() {
        if unit.path.is_none() {
            continue;
        }
        let path = std::mem::replace(&mut unit.path, None);
        let (mut current_waypoint, path) = path.unwrap();

        let waypoint_current = path[current_waypoint as usize];
        let waypoint_next = path[current_waypoint as usize + 1];

        let mut progress = match unit.path_progress {
            Some(progress) => progress,
            None => {
                // unit is starting path here

                let dir = IsometricDirection::from_vec((
                    waypoint_next.0 - waypoint_current.0,
                    waypoint_next.1 - waypoint_current.1,
                ))
                .unwrap();
                let animation = match dir {
                    IsometricDirection::UpRight => unit.move_up_right.clone(),
                    IsometricDirection::UpLeft => unit.move_up_left.clone(),
                    IsometricDirection::DownRight => unit.move_down_right.clone(),
                    IsometricDirection::DownLeft => unit.move_down_left.clone(),
                };
                animatable.play(animation, true);
                0.
            }
        };

        if unit.render_priority.is_none() {
            let prio_1 = iso_transform(
                waypoint_current.0 as f32,
                waypoint_current.1 as f32,
                map_layout.tiles[&(waypoint_current.0, waypoint_current.1)] as f32,
                1.,
                1.,
                true,
            )
            .z;
            let prio_2 = iso_transform(
                waypoint_next.0 as f32,
                waypoint_next.1 as f32,
                map_layout.tiles[&(waypoint_next.0, waypoint_next.1)] as f32,
                1.,
                1.,
                true,
            )
            .z;
            let render_prio = max(prio_1, prio_2);
            unit.render_priority = Some(render_prio);
        }

        // update progress
        progress += unit.travel_speed * time.delta().as_millis() as f32 / 1000.;

        // unit has reached a waypoint
        if progress > 1.0 {
            current_waypoint += 1;
            unit.render_priority = None;
            unit.path_progress = None;
            // without this line unit gets set to next waypoint for 1 frame
            progress = 0.;
            if current_waypoint as usize == path.len() - 1 {
                let last_waypoint = path.last().unwrap();
                unit.x = last_waypoint.0 as f32;
                unit.y = last_waypoint.1 as f32;
                unit_registry.units.remove(&path[0]);
                unit_registry.units.insert(*last_waypoint, entity);
                map_state.unit_moving = false;
                animatable.play(unit.idle.clone(), true);
                continue;
            }
        } else {
            unit.path_progress = Some(progress);
        }

        // update position
        unit.x = (1. - progress) * path[current_waypoint as usize].0 as f32
            + progress * path[current_waypoint as usize + 1].0 as f32;
        unit.y = (1. - progress) * path[current_waypoint as usize].1 as f32
            + progress * path[current_waypoint as usize + 1].1 as f32;

        unit.path = Some((current_waypoint, path));
    }
}
