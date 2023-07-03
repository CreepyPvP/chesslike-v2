use bevy::{prelude::{Plugin, Res, IntoSystemConfig, Component, Query, With, Entity, Color}, sprite::Sprite, render::color};

use super::{picking::PickState, GameSystemSets};

#[derive(Component)]
pub struct Hoverable;

pub struct HoverablePlugin;

impl Plugin for HoverablePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(update_tint.after(GameSystemSets::Input));
    }
}

fn update_tint(pick_state: Res<PickState>, mut hoverables: Query<(&mut Sprite, Entity), With<Hoverable>>) {
    for (mut sprite, entity) in hoverables.iter_mut() {
        let mut color = Color::WHITE;
        if let Some(selected) = pick_state.selected { 
            if selected == entity {
                color = Color::rgb(1.2, 1.2, 1.2);
            }
        } 

        sprite.color = color;
    }
}
