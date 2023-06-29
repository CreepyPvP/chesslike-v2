use bevy::prelude::{Handle, HandleUntyped, Image, Resource};

use crate::assets::types::{TiledMap, TiledSet};

#[derive(Resource)]
pub struct GameConfig {
    pub tileset: String,
    pub map: String,
}

#[derive(Resource)]
pub struct GameAssets {
    pub map: Handle<TiledMap>,
    pub tileset: Handle<TiledSet>,
    pub tiles: Option<Vec<Handle<Image>>>,

    pub assets: Option<Vec<HandleUntyped>>,
}
