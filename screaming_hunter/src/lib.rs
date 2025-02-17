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
        use level::system::{MAX_X_POSITION, MAX_Y_POSITION, MIN_X_POSITION, MIN_Y_POSITION};

        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Screaming Hunter".to_string(),
                resolution: WindowResolution::new(
                    MIN_X_POSITION.abs() + MAX_X_POSITION,
                    MIN_Y_POSITION.abs() + MAX_Y_POSITION,
                ),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading).continue_to_state(MyStates::Next),
        )
        .add_plugins((
            debug::DebugPlugin,
            batt::BattPlugin,
            physic::PhysicPlugin,
            dev_tool::DevToolPlugin,
            level::LevelPlugin,
        ));
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, States, Hash)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}
