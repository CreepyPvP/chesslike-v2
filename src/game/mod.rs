use bevy::prelude::*;

use crate::AppState;

use self::{
    animation::AnimatorPlugin, game_state::GameStatePlugin, map::MapPlugin, picking::PickingPlugin,
    unit::UnitPlugin,
};

mod animation;
pub mod game_state;
mod isometric;
pub mod map;
pub mod picking;
mod unit;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum GameSystemSets {
    Input,
    Logic,
    Update,
    Render,
}

pub enum GameEvent {
    SpawnUnit(i32, i32),
    SpawnedUnit(Entity),
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<GameEvent>();
        app.configure_set(GameSystemSets::Input.in_set(OnUpdate(AppState::Game)));
        app.configure_set(
            GameSystemSets::Logic
                .after(GameSystemSets::Input)
                .in_set(OnUpdate(AppState::Game)),
        );
        app.configure_set(
            GameSystemSets::Update
                .after(GameSystemSets::Logic)
                .in_set(OnUpdate(AppState::Game)),
        );
        app.configure_set(
            GameSystemSets::Render
                .after(GameSystemSets::Update)
                .in_set(OnUpdate(AppState::Game)),
        );

        app.add_plugin(AnimatorPlugin);
        app.add_plugin(MapPlugin);
        app.add_plugin(GameStatePlugin);
        app.add_plugin(UnitPlugin);
        app.add_plugin(PickingPlugin);
    }
}
