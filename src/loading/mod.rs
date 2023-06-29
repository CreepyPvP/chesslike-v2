use bevy::{asset::Asset, prelude::*};

use crate::{
    assets::types::{TiledMap, TiledSet, TiledTile},
    types::{GameAssets, GameConfig},
    AppState,
};

#[derive(Resource)]
struct LoadingResource {
    map: Handle<TiledMap>,
    tileset: Handle<TiledSet>,
    tiles: Option<Vec<Handle<Image>>>,

    all: Vec<HandleUntyped>,
}

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_loading.in_schedule(OnEnter(AppState::Loading)));
        app.add_system(load.in_set(OnUpdate(AppState::Loading)));
    }
}

fn start_loading(mut command: Commands, assets: Res<AssetServer>, game_config: Res<GameConfig>) {
    let tileset_h: Handle<TiledSet> = assets.load(&game_config.tileset);
    let map_h: Handle<TiledMap> = assets.load(&game_config.tileset);

    let resource = LoadingResource {
        all: vec![map_h.clone_untyped(), tileset_h.clone_untyped()],
        map: map_h,
        tileset: tileset_h,
        tiles: None,
    };

    command.insert_resource(resource);
}

fn load(
    assets: Res<AssetServer>,
    tilesets: Res<Assets<TiledSet>>,
    mut loading: ResMut<LoadingResource>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let tileset = match tilesets.get(&loading.tileset) {
        Some(tileset) => tileset,
        None => return,
    };

    if loading.tiles.is_none() {
        loading.tiles = Some(
            tileset
                .tiles
                .iter()
                .map(|tile| {
                    let tile = assets.load(tile.path());
                    loading.all.push(tile.clone_untyped());
                    tile
                })
                .collect(),
        );
    }

    for item in &loading.all {
        match assets.get_load_state(item) {
            bevy::asset::LoadState::Loaded => (),
            _ => return,
        }
    }

    next_state.set(AppState::Game);
}
