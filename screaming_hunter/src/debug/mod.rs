use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod clap_debug;
mod ressource;
mod system;

/// Plugin for Debuging with argument when using [`cargo run --`] or [`the_executable`]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(clap_debug::ClapPlugin::<ressource::DebugConfig>::default())
            .init_resource::<ressource::AssetsLoading>()
            .add_systems(PreStartup, system::setup)
            .add_systems(
                Update,
                system::check_assets_ready.run_if(|world: &World| {
                    if let Some(debug_config) = world.get_resource::<ressource::DebugConfig>() {
                        return debug_config.show_asset_loading_state;
                    };
                    false
                }),
            )
            .add_plugins(WorldInspectorPlugin::new());
    }
}
