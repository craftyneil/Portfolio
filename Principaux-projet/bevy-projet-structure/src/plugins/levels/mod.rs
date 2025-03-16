use bevy::prelude::*;

mod systems;

pub(super) struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_test_room).add_systems(Update, systems::clamp_position_to_room_size);
    }
}
