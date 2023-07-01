use bevy::prelude::*;

use crate::{AppState, assets::types::TiledMap, game_config::GameAssets};

use super::isometric::iso_transform;


pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_units.in_schedule(OnEnter(AppState::Game)));
    }
}

#[derive(Component)]
pub struct Unit {

}

fn create_units(mut commands: Commands, tilemaps: Res<Assets<TiledMap>>, game_assets: Res<GameAssets>) {
    let tilemap = tilemaps.get(&game_assets.map).unwrap();
    let (tile_w, tile_h) = (tilemap.tilewidth as f32, tilemap.tileheight as f32);

    commands.spawn((
        // Unit {
        //     top_right: assets.load("vehicles/PNG/Police/police_NE.png"),
        //     top_left: assets.load("vehicles/PNG/Police/police_NW.png"),
        //     bot_right: assets.load("vehicles/PNG/Police/police_SE.png"),
        //     bot_left: assets.load("vehicles/PNG/Police/police_SW.png"),
        // },
        SpriteBundle {
            texture: game_assets.units.get("police").unwrap().clone(),
            transform: iso_transform(3., 4., 0., tile_w, tile_h).with_scale(Vec3::new(2.,2.,2.)),
            ..default()
        },
    ));
}
