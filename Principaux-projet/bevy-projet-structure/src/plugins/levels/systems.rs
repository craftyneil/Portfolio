use bevy::prelude::*;

pub const MINIMUM_X_POSITION: f32 = -400.0;
pub const MAXIMUM_X_POSITION: f32 = 400.0;
pub const MINIMUM_Y_POSITION: f32 = -400.0;
pub const MAXIMUM_Y_POSITION: f32 = 400.0;

pub(super) fn setup_test_room(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub(super) fn clamp_position_to_room_size(mut query: Query<&mut Transform>) {
    for mut transform in &mut query {
        transform.translation.x = transform
            .translation
            .x
            .clamp(MINIMUM_X_POSITION, MAXIMUM_X_POSITION);
        transform.translation.y = transform
            .translation
            .y
            .clamp(MINIMUM_Y_POSITION, MAXIMUM_Y_POSITION);
    }
}
