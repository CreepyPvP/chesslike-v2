use bevy::{asset::{AssetLoader, LoadedAsset}, reflect::TypeUuid};

use super::types::TiledMap;

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
            Ok(()) }
        )
    }

    fn extensions(&self) -> &[&str] {
        &["tmj"]
    }
}
