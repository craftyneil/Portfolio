use bevy::prelude::{Component, Sprite};

use crate::physic::component::Velocity;

#[derive(Debug, Component)]
#[require(Sprite, Velocity)]
pub struct Batt;
