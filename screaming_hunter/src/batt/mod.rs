use bevy::{
    app::PreUpdate,
    prelude::{OnEnter, Plugin},
};
use bevy_asset_loader::prelude::*;

use crate::MyStates;

mod component;
mod ressource;
mod system;

pub struct BattPlugin;

impl Plugin for BattPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_loading_state(
            LoadingStateConfig::new(MyStates::AssetLoading)
                .load_collection::<ressource::BattAsset>(),
        )
        .add_systems(OnEnter(MyStates::Next), system::setup_batt)
        .add_systems(PreUpdate, system::update_batt_velocity);
    }
}
