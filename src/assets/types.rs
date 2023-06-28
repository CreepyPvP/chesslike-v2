use bevy::reflect::TypeUuid;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TiledLayer {
    pub data: Vec<i32>,
    pub id: i32,
    pub name: String,
    pub height: u32,
    pub width: u32,
}

#[derive(Deserialize, Debug, TypeUuid)]
#[uuid = "1cb1e503-3c34-4f38-ab9e-2338e2c4a0f4"]
pub struct TiledMap {
    pub compressionlevel: i32,
    pub height: u32,
    pub width: u32,
    pub layers: Vec<TiledLayer>,
    pub tilewidth: u32,
    pub tileheight: u32,
}
