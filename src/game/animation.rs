use bevy::{
    prelude::{Component, IntoSystemConfig, Plugin, Query, Rect, Res, Vec2},
    sprite::Sprite,
    time::{Time, Timer},
};

use super::GameSystemSets;

#[derive(Clone)]
pub struct Animation {
    repeating: bool,
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
        repeating: bool,
    ) -> Self {
        let frames: Vec<Rect> = frames
            .iter()
            .map(|frame| {
                let top_left = Vec2::new(
                    (frame.0 * (frame_w + extrusion_x)) as f32,
                    (frame.1 * (frame_h + extrusion_y)) as f32,
                );
                let bot_right = Vec2::new(top_left.x + frame_w as f32, top_left.y + frame_h as f32);
                Rect {
                    min: top_left,
                    max: bot_right,
                }
            })
            .collect();

        Animation {
            timer: Timer::from_seconds(frame_duration_seconds, bevy::time::TimerMode::Repeating),
            frames,
            repeating,
        }
    }
}

#[derive(Component)]
pub struct Animatable {
    current_animation: Animation,
    next_animation: Option<Animation>,
    current_frame: usize,
    active: bool,
}

impl Animatable {
    pub fn from_anim(animation: Animation) -> Self {
        Animatable {
            current_animation: animation,
            current_frame: 0,
            active: true,
            next_animation: None,
        }
    }

    pub fn stop(&mut self, instant: bool) {
        if instant {
            self.active = false;
        } else {
            self.current_animation.repeating = false;
        }
    }

    pub fn play(&mut self, anim: Animation, force: bool) {
        if force {
            self.current_animation = anim;
            self.current_frame = 0;
            self.active = true;
        } else {
            self.next_animation = Some(anim);
            self.current_animation.repeating = false;
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
        sprite.rect = Some(animatable.current_animation.frames[animatable.current_frame]);

        if !animatable.active {
            return;
        }

        if animatable
            .current_animation
            .timer
            .tick(time.delta())
            .just_finished()
        {
            if animatable.current_frame >= animatable.current_animation.frames.len() - 1 {
                if animatable.current_animation.repeating {
                    animatable.current_frame = 0;
                }
                if let Some(next) = std::mem::replace(&mut animatable.next_animation, None) {
                    animatable.current_animation = next;
                    animatable.current_frame = 0;
                }
            } else {
                animatable.current_frame = animatable.current_frame + 1;
            }
        }
    }
}
