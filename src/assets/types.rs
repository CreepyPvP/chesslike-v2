use bevy::reflect::TypeUuid;
use serde::Deserialize;
use serde::*;

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

#[derive(Deserialize, Serialize, Debug)]
pub struct TiledTile {
    pub id: u32,
    pub image: String,
}

impl TiledTile {
    pub fn path(&self) -> String {
        String::from("tileset/prototype/Tiles/") + &self.image[9..]
    }
}

#[derive(Deserialize, Debug, TypeUuid)]
#[uuid = "78c1a660-ed27-4d62-ab29-ce24d90279a0"]
pub struct TiledSet {
    pub name: String,
    pub tilecount: u32,
    pub tiles: Vec<TiledTile>,
    pub tileheight: u32,
    pub tilewidth: u32,
}
