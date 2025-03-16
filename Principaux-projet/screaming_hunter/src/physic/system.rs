use bevy::{
    math::Vec3,
    prelude::{Query, Transform},
};

use super::component::Velocity;

pub fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        transform.translation += velocity.value;

        // prevent an acceleration of the batt
        velocity.value = Vec3::ZERO;
    }
}
