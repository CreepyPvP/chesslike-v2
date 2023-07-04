use bevy::prelude::*;

use crate::AppState;

use self::{animation::AnimatorPlugin, map::MapPlugin, picking::PickingPlugin, unit::UnitPlugin};

mod animation;
mod isometric;
pub mod map;
pub mod picking;
mod unit;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum GameSystemSets {
    Logic,
    Render,
    Input,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_set(GameSystemSets::Input.in_set(OnUpdate(AppState::Game)));
        app.configure_set(
            GameSystemSets::Logic
                .after(GameSystemSets::Input)
                .in_set(OnUpdate(AppState::Game)),
        );
        app.configure_set(
            GameSystemSets::Render
                .after(GameSystemSets::Logic)
                .in_set(OnUpdate(AppState::Game)),
        );

        app.add_plugin(AnimatorPlugin);
        app.add_plugin(MapPlugin);
        app.add_plugin(UnitPlugin);
        app.add_plugin(PickingPlugin);
    }
}
