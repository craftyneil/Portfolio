use bevy::prelude::*;

mod camera;
mod levels;
mod player;
mod debug;

pub struct BevyPlugin;

impl Plugin for BevyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(player::BattPlugin)
            .add_plugins(levels::LevelsPlugin);
        
        #[cfg(debug_assertions)]
        app.add_plugins(debug::DebugPlugin);
    }
}
