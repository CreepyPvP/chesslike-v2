use assets::AssetsPlugin;
use bevy::prelude::*;
use game::GamePlugin;
use loading::LoadingPlugin;
use main_menu::MainMenuPlugin;
use types::GameConfig;

mod assets;
mod game;
mod loading;
mod main_menu;
mod types;

#[derive(Debug, Default, Clone, Eq, States, PartialEq, Hash)]
pub enum AppState {
    Menu,
    #[default]
    Loading,
    Game,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .insert_resource(GameConfig {
            map: "tilemap/1.tmj".to_string(),
            tileset: "tileset/prototype/Map/map_tiles.tsj".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetsPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .add_startup_system(spawn_camera)
        .run()
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 1000.),
        ..Default::default()
    });
}
