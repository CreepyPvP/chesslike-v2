use bevy::prelude::{Camera2dBundle, Commands, Plugin};

use self::player::PlayerPlugin;

mod player;
mod level;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup).add_plugin(PlayerPlugin);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
