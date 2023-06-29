use bevy::prelude::*;

use crate::AppState;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_units.in_schedule(OnEnter(AppState::Game)));
    }
}

#[derive(Component)]
pub struct Unit {
    top_left: Handle<Image>,
    top_right: Handle<Image>,
    bot_left: Handle<Image>,
    bot_right: Handle<Image>,
}

fn create_units(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Unit {
            top_right: assets.load("vehicles/PNG/Police/police_NE.png"),
            top_left: assets.load("vehicles/PNG/Police/police_NW.png"),
            bot_right: assets.load("vehicles/PNG/Police/police_SE.png"),
            bot_left: assets.load("vehicles/PNG/Police/police_SW.png"),
        },
        SpriteBundle {
            texture: assets.load("vehicles/PNG/Police/police_NE.png"),
            transform: Transform::from_xyz(0., 0., 900.),
            ..default()
        },
    ));
}
