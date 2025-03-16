use bevy::prelude::*;

use super::movement::Velocity;

pub struct PositionClampPlugin;

impl Plugin for PositionClampPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, clamp_entities);
    }
}

fn clamp_entities(windows: Query<&Window>, mut entities: Query<&mut Transform, With<Velocity>>) {
    for window in windows.iter() {
        for mut transform in entities.iter_mut() {
            transform.translation.x = transform
                .translation
                .x
                .clamp(-window.width() / 2.0, window.width() / 2.0);

            transform.translation.y = transform
                .translation
                .y
                .clamp(-window.height() / 2.0, window.height() / 2.0);
        }
    }
}
