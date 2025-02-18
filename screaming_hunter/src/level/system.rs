use bevy::{
    math::{Vec2, Vec3Swizzles},
    prelude::{Camera2d, Commands, Query, Transform, With},
};

use crate::physic::component::Velocity;
pub const MIN_POSITION: Vec2 = Vec2::new(-400f32, -400f32);
pub const MAX_POSITION: Vec2 = Vec2::new(400f32, 400f32);

pub fn setup_room_level(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn clamp_entities_to_room_size(mut query: Query<&mut Transform, With<Velocity>>) {
    for mut transform in &mut query {
        transform.translation = transform
            .translation
            .xy()
            .clamp(MIN_POSITION, MAX_POSITION)
            .extend(0f32);
    }
}
