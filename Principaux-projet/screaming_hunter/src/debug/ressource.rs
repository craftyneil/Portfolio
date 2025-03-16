use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

#[derive(Debug, clap::Parser, Resource)]
pub struct DebugConfig {
    #[arg(long = "asset_state")]
    pub show_asset_loading_state: bool,
}
