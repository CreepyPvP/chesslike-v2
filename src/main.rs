use bevy::prelude::*;
use plugins::AppPlugin;

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AppPlugin)
        .run()
}
