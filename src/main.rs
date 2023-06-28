use assets::AssetsPlugin;
use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

mod assets;
mod game;
mod main_menu;

#[derive(Debug, Default, Clone, Eq, States, PartialEq, Hash)]
pub enum AppState {
    Menu,
    Loading,
    #[default]
    Game,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetsPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .add_startup_system(spawn_camera)
        .run()
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}

