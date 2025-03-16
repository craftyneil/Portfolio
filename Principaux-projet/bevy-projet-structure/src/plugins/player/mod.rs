use bevy::prelude::*;

mod components;
mod systems;

pub struct BattPlugin;

impl Plugin for BattPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_batt)
            .add_systems(Update, systems::move_batt);
    }
}
