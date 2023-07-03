use bevy::prelude::*;

use self::{
    animation::AnimatorPlugin, hoverable::HoverablePlugin, map::MapPlugin, picking::PickingPlugin,
    unit::UnitPlugin,
};

mod animation;
mod hoverable;
mod isometric;
mod map;
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
        app.configure_set(GameSystemSets::Input);
        app.configure_set(GameSystemSets::Logic.after(GameSystemSets::Input));
        app.configure_set(GameSystemSets::Render.after(GameSystemSets::Logic));

        app.add_plugin(HoverablePlugin);
        app.add_plugin(AnimatorPlugin);
        app.add_plugin(MapPlugin);
        app.add_plugin(UnitPlugin);
        app.add_plugin(PickingPlugin);
    }
}
