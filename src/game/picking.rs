use bevy::{
    prelude::{
        Camera, Component, GlobalTransform, IntoSystemConfig, Plugin, Query,
        With, Vec2, Transform, Entity, Commands,
    },
    render::camera::RenderTarget,
    window::{PrimaryWindow, Window},
};

use crate::util::collisions::Triangle;

use super::GameSystemSets;

#[derive(Component, Clone)]
pub struct Pickable {
    pub triangles: Vec<Triangle>
}

#[derive(Component, Default)]
pub struct PickCamera;

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(pick_input.in_set(GameSystemSets::Input));
    }
}

fn pick_input(
    camera: Query<(&Camera, &GlobalTransform)>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    pickables: Query<(&Pickable, &GlobalTransform, Entity)>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera.single();
    // fuck off bevy docs
    let window = match camera.target {
        RenderTarget::Window(bevy::window::WindowRef::Primary) => primary_window.single(),
        // Ignore this
        // RenderTarget::Window(bevy::window::WindowRef::Entity(entity)) => windows.get(entity),
        _ => return,
    };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            if let Some(entity) = pick_nearst(&pickables, &world_pos) {
                commands.get_entity(entity).unwrap().despawn();
            }
        }
    }
}

fn pick_nearst(pickables: &Query<(&Pickable, &GlobalTransform, Entity)>, world_pos: &Vec2) -> Option<Entity> {
    for (pickable, transform, entity) in &(*pickables) {
        let obj_translation = transform.translation();
        let corrected_pos = Vec2::new(world_pos.x - obj_translation.x, world_pos.y - obj_translation.y);
        for triangle in &pickable.triangles {
            if triangle.contains(&corrected_pos) {
                return Some(entity);
            }
        }
    }

    None
}
