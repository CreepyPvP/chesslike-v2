use bevy::prelude::*;

use self::{map::MapPlugin, unit::UnitPlugin};

mod map;
mod unit;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum GameSystemSets {
    Start,
    Input,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_set(GameSystemSets::Start);
        app.configure_set(GameSystemSets::Input);

        app.add_plugin(MapPlugin);
        app.add_plugin(UnitPlugin);
    }
}
