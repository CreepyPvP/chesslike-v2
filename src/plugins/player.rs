use std::hash::BuildHasherDefault;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(create_player)
            .add_system(do_player_movement);
    }
}

fn create_player(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: assets.load("icon.png"),
            transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.)),
            ..default()
        },
        PlayerMovement,
    ));
}

#[derive(Component)]
pub struct PlayerMovement;

fn do_player_movement(
    mut query: Query<&mut Transform, With<PlayerMovement>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let movement_speed = 340.0;
    let mut dir = Vec2::default();
    if input.pressed(KeyCode::W) {
        dir.y += 1.;
    }
    if input.pressed(KeyCode::S) {
        dir.y -= 1.;
    }
    if input.pressed(KeyCode::A) {
        dir.x -= 1.;
    }
    if input.pressed(KeyCode::D) {
        dir.x += 1.;
    }
    let dir = dir.normalize_or_zero();

    for mut entity in &mut query {
        entity.translation.x += movement_speed * dir.x * time.delta_seconds();
        entity.translation.y += movement_speed * dir.y * time.delta_seconds();
    }
}
