use bevy::prelude::*;

use crate::{assets::types::TiledMap, game_config::GameAssets, AppState};

use super::{isometric::iso_transform, animation::{Animation, Animatable}};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_units.in_schedule(OnEnter(AppState::Game)));
    }
}

#[derive(Component)]
pub struct Unit {}

fn create_units(
    mut commands: Commands,
    tilemaps: Res<Assets<TiledMap>>,
    game_assets: Res<GameAssets>,
) {
    let tilemap = tilemaps.get(&game_assets.map).unwrap();
    let (tile_w, tile_h) = (tilemap.tilewidth as f32, tilemap.tileheight as f32);

    // this is incredibly ugly...
    let ogre_walk = Animation::new(0.4, 192, 192, 64, 64, vec!((0, 5), (1, 5), (2, 5), (3, 5)));

    commands.spawn((
        SpriteBundle {
            texture: game_assets.units.get("ogre").unwrap().clone(),
            transform: iso_transform(1., 1., 1., tile_w, tile_h, true).with_scale(Vec3::new(0.5, 0.5, 0.5)),
            ..default()
        },
        Animatable::from_anim(ogre_walk, true),
    ));
}
