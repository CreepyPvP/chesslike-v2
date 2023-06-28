use crate::AppState;
use bevy::prelude::*;

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

        app.add_systems((
            create_map
                .in_schedule(OnEnter(AppState::Game))
                .in_set(GameSystemSets::Start),
            destroy_map
                .in_schedule(OnExit(AppState::Game))
        ));
    }
}

#[derive(Component)]
struct Map;

fn create_map(mut commands: Commands, assests: Res<AssetServer>) {
    // let map = commands.spawn((
    //     SpriteBundle {
    //         texture: assests.load("icon.png"),
    //         ..default()
    //     },
    //     Map,
    // ));
}

fn destroy_map(mut command: Commands, query: Query<Entity, With<Map>>) {
    if let Ok(map_entity) = query.get_single() {
        command.entity(map_entity).despawn_recursive();
    }
}
