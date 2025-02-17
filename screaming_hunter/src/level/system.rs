use bevy::{
    math::{Vec2, Vec3Swizzles},
    prelude::{Camera2d, Commands, Query, Transform, With},
};

use crate::physic::component::Velocity;

pub const MIN_X_POSITION: f32 = -400f32;
pub const MAX_X_POSITION: f32 = 400f32;
pub const MIN_Y_POSITION: f32 = -400f32;
pub const MAX_Y_POSITION: f32 = 400f32;

pub fn setup_room_level(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn clamp_entities_to_room_size(mut query: Query<&mut Transform, With<Velocity>>) {
    for mut transform in &mut query {
        transform.translation = transform
            .translation
            .xy()
            .clamp(
                Vec2::new(MIN_X_POSITION, MIN_Y_POSITION),
                Vec2::new(MAX_X_POSITION, MAX_Y_POSITION),
            )
            .extend(0f32);
    }
}
