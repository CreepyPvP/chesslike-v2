use bevy::prelude::*;

use self::{types::*, tiled_loader::TiledMapLoader};

pub mod types;
mod tiled_loader;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<TiledMap>()
            .init_asset_loader::<TiledMapLoader>();
    }
}
