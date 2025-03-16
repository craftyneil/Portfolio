use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::general_component::movement::Velocity;
///marker for the player
#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }
    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

fn attack(mut player: Query<(&mut Transform, &mut Sprite), With<Player>>) {}

fn trigger_animation<S: Component>(mut query: Query<&mut AnimationConfig, With<S>>) {
    let mut animation = query.single_mut();
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (attack).run_if(input_just_pressed(MouseButton::Left)),
        );
    }
}
