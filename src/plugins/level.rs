use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_level);
    }
}

fn generate_level(mut commands: Commands) {
     
}
