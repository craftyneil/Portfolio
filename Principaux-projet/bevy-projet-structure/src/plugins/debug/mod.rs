use bevy::prelude::*;

mod systems;
mod ressource;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ressource::AssetsLoading>().add_systems(Startup, systems::setup).add_systems(Update, systems::check_assets_ready);
    }
}