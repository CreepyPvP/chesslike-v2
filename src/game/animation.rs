use bevy::{
    prelude::{Commands, Component, Entity, Handle, Image, IntoSystemConfig, Plugin, Query, Rect, Vec2, Res},
    sprite::Sprite, time::{Timer, Time},
};

use super::GameSystemSets;

pub struct Animation {
    timer: Timer,
    frames: Vec<Rect>,
}

impl Animation {
    pub fn new(
        frame_duration_seconds: f32,
        frame_w: u32,
        frame_h: u32,
        extrusion_x: u32,
        extrusion_y: u32,
        frames: Vec<(u32, u32)>,
    ) -> Self {
        let frames: Vec<Rect> = frames.iter().map(|frame| {
            let top_left = Vec2::new((frame.0 * (frame_w + extrusion_x)) as f32, (frame.1 * (frame_h + extrusion_y)) as f32);
            let bot_right = Vec2::new(top_left.x + frame_w as f32, top_left.y + frame_h as f32);
            Rect { 
                min: top_left,
                max: bot_right,
            }
        }).collect();

        Animation {
            timer: Timer::from_seconds(frame_duration_seconds, bevy::time::TimerMode::Repeating),
            frames,
        }
    }
}

#[derive(Component)]
pub struct Animatable {
    current: Animation,
    repeat: bool,
    current_frame: usize,
}

impl Animatable {
    pub fn from_anim(animation: Animation, repeat: bool) -> Self {
        Animatable {
            current: animation,
            repeat,
            current_frame: 0,
        }
    }
}

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(update_animations.in_set(GameSystemSets::Render));
    }
}

fn update_animations(mut animatables: Query<(&mut Animatable, &mut Sprite)>, time: Res<Time>) {
    for (mut animatable, mut sprite) in animatables.iter_mut() {
        sprite.rect = Some(animatable.current.frames[animatable.current_frame]);

        if animatable.current.timer.tick(time.delta()).just_finished() {
            animatable.current_frame = (animatable.current_frame + 1) % animatable.current.frames.len();
        }
    }
}
