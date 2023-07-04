use std::collections::HashMap;

use bevy::prelude::*;

use crate::{assets::types::TiledMap, game_config::GameAssets, AppState};

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
}

impl Unit {}

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
                ..default()
            },
            Unit {
                travel_distance: 2,
                x: 1.,
                y: 1.,
                z: 1.,
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
