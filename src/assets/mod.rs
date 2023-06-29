use bevy::prelude::*;

use self::{
    tiled_loader::{TiledMapLoader, TiledSetLoader},
    types::*,
};

mod tiled_loader;
pub mod types;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TiledMap>()
            .init_asset_loader::<TiledMapLoader>()
            .add_asset::<TiledSet>()
            .init_asset_loader::<TiledSetLoader>();
    }
}
