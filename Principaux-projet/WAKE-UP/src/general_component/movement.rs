use std::ops::RangeInclusive;

use bevy::prelude::*;
use rand::Rng;

const RANDOM_MOVEMENT_RANGE: RangeInclusive<f32> = 0.0..=(std::f32::consts::TAU);

pub struct MovementPlugin;

#[derive(Debug, Component)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3 { x, y, z },
        }
    }
}

#[derive(Debug, Component, Default)]
pub struct Speed {
    pub value: f32,
}

#[derive(Debug, Component)]
pub struct NoRandomMovement;

#[derive(Debug, Component)]
pub struct RandomMovement;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((move_forward, move_randomly), process_movement).chain(),
        );
    }
}

fn process_movement(
    mut entities: Query<(&mut Transform, &Velocity, Option<&mut Sprite>)>,
    time: Res<Time>,
) {
    for (mut transform, velocity, potential_sprite) in entities.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();

        if let Some(mut sprite) = potential_sprite {
            if velocity.value.x > 0.0 {
                sprite.flip_y = false;
            } else if velocity.value.x < 0.0 {
                sprite.flip_y = true;
            }
        }
    }
}

fn move_forward(mut entities: Query<(&Transform, &mut Velocity, &Speed), With<NoRandomMovement>>) {
    for (transform, mut velocity, speed) in entities.iter_mut() {
        velocity.value = transform.forward() * speed.value;
    }
}

fn move_randomly(mut entities: Query<(&mut Velocity, &Speed), With<RandomMovement>>) {
    for (mut velocity, speed) in entities.iter_mut() {
        let random_angle = rand::thread_rng().gen_range(RANDOM_MOVEMENT_RANGE);
        velocity.value = Vec3::new(random_angle.cos(), random_angle.sin(), 0.0) * speed.value;
    }
}
