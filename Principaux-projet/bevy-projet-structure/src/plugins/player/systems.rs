use bevy::prelude::*;

use super::components::{Batt, Velocity};

const BATT_SPEED: f32 = 5.0;

pub(super) fn setup_batt(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Batt,
        Sprite::from_image(asset_server.load("batt/static_batt.png")),
        Transform::default().with_scale(Vec3::new(100.0, 100.0, 0.0).normalize() * 0.5),
    ));
}

pub(super) fn move_batt(
    batt: Single<(&mut Transform, &mut Velocity), With<Batt>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut batt_transform, mut batt_velocity) = batt.into_inner();

    // reset the velocity to prevent an acceleration of the batt when moving
    batt_velocity.value = Vec3::ZERO;

    if keys.pressed(KeyCode::ArrowDown) {
        batt_velocity.value.y -= BATT_SPEED * time.delta_secs();
    }

    if keys.pressed(KeyCode::ArrowUp) {
        batt_velocity.value.y += BATT_SPEED * time.delta_secs();
    }

    if keys.pressed(KeyCode::ArrowRight) {
        batt_velocity.value.x += BATT_SPEED * time.delta_secs();
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        batt_velocity.value.x -= BATT_SPEED * time.delta_secs();
    }

    batt_transform.translation += batt_velocity.value.normalize_or(Vec3::ZERO) * BATT_SPEED;
}
