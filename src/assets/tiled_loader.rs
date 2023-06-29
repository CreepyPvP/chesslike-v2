use bevy::asset::{AssetLoader, LoadedAsset};

use super::types::{TiledMap, TiledSet};

#[derive(Default)]
pub struct TiledMapLoader;

impl AssetLoader for TiledMapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let map: TiledMap = serde_json::from_slice(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(map));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmj"]
    }
}

#[derive(Default)]
pub struct TiledSetLoader;

impl AssetLoader for TiledSetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let set: TiledSet = serde_json::from_slice(bytes)?;
            let asset = LoadedAsset::new(set.clone());
            let asset =
                asset.with_dependencies(set.tiles.iter().map(|tile| tile.path().into()).collect());
            load_context.set_default_asset(asset);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tsj"]
    }
}
