use std::collections::HashMap;

use bevy::prelude::*;

use crate::{assets::types::TiledMap, game_config::GameAssets, AppState};

use super::{
    animation::{Animatable, Animation},
    isometric::iso_transform,
};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitRegistry::default());
        app.add_system(create_units.in_schedule(OnEnter(AppState::Game)));
    }
}

#[derive(Resource, Default)]
pub struct UnitRegistry {
    pub units: HashMap<(i32, i32), Entity>,
}

#[derive(Component)]
pub struct Unit {}

fn create_units(
    mut commands: Commands,
    tilemaps: Res<Assets<TiledMap>>,
    game_assets: Res<GameAssets>,
    mut unit_registry: ResMut<UnitRegistry>,
) {
    let tilemap = tilemaps.get(&game_assets.map).unwrap();
    let (tile_w, tile_h) = (tilemap.tilewidth as f32, tilemap.tileheight as f32);

    // this is incredibly ugly...
    let ogre_walk = Animation::new(0.4, 192, 192, 64, 64, vec![(0, 5), (1, 5), (2, 5), (3, 5)]);

    let ogre = commands
        .spawn((
            SpriteBundle {
                texture: game_assets.units.get("ogre").unwrap().clone(),
                transform: iso_transform(1., 1., 1., tile_w, tile_h, true)
                    .with_scale(Vec3::new(0.5, 0.5, 0.5)),
                ..default()
            },
            Animatable::from_anim(ogre_walk, true),
        ))
        .id();
    unit_registry.units.insert((1, 1), ogre);
}
