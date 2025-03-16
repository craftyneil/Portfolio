use bevy::{prelude::*, window::WindowResolution};
use bevy_asset_loader::prelude::*;

mod batt;
mod debug;
mod dev_tool;
mod level;
mod physic;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        use level::system::{MAX_POSITION, MIN_POSITION};

        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Screaming Hunter".to_string(),
                resolution: WindowResolution::new(
                    MIN_POSITION.x.abs() + MAX_POSITION.x, // sum the boundaries to get the window size in x
                    MIN_POSITION.y.abs() + MAX_POSITION.y, // same but with y coordinates
                ),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading).continue_to_state(MyStates::Next),
        )
        .add_plugins((batt::BattPlugin, physic::PhysicPlugin, level::LevelPlugin));

        #[cfg(debug_assertions)]
        app.add_plugins((debug::DebugPlugin, dev_tool::DevToolPlugin));
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, States, Hash)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}
