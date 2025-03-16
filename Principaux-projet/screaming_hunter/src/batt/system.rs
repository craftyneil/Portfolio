use bevy::{
    input::ButtonInput,
    math::Vec2,
    prelude::{Commands, KeyCode, Res, Single, Transform, With},
    sprite::Sprite,
    time::Time,
};

use super::super::physic::component::Velocity;
use super::component::Batt;
use super::ressource::BattAsset;

const BATT_SPEED: f32 = 10.0;
const BATT_SCALE_FACTOR: f32 = 0.5;

pub fn setup_batt(mut commands: Commands, batt_asset: Res<BattAsset>) {
    commands.spawn((
        Batt,
        Sprite::from_image(batt_asset.batt_image.clone()),
        Transform::from_scale(Vec2::ONE.extend(0f32).normalize() * BATT_SCALE_FACTOR),
    ));
}

pub fn update_batt_velocity(
    player: Single<&mut Velocity, With<Batt>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_velocity = player.into_inner();

    if keys.pressed(KeyCode::ArrowDown) {
        player_velocity.value.y -= BATT_SPEED * time.delta_secs();
    }

    if keys.pressed(KeyCode::ArrowUp) {
        player_velocity.value.y += BATT_SPEED * time.delta_secs();
    }

    if keys.pressed(KeyCode::ArrowRight) {
        player_velocity.value.x += BATT_SPEED * time.delta_secs();
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        player_velocity.value.x -= BATT_SPEED * time.delta_secs();
    }

    player_velocity.normalize_to(BATT_SPEED);
}
