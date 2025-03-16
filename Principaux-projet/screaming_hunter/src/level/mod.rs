use bevy::app::{Plugin, PostUpdate, Startup};

pub mod system;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, system::setup_room_level)
            .add_systems(PostUpdate, system::clamp_entities_to_room_size);
    }
}
