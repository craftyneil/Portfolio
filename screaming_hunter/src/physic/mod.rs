use bevy::{app::Update, prelude::Plugin};

pub mod component;
pub mod system;

pub struct PhysicPlugin;

impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, system::apply_velocity);
    }
}
