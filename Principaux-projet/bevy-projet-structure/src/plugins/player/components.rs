use bevy::prelude::*;

#[derive(Component)]
#[require(Sprite, Velocity)]
pub(super) struct Batt;

#[derive(Debug, Default, Component)]
pub(super) struct Velocity {
    pub(super) value: Vec3
}