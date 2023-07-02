use std::collections::HashMap;

use bevy::prelude::{Handle, Image, Resource};

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
    pub tiles: Vec<Handle<Image>>,
    pub units: HashMap<String, Handle<Image>>,
}
