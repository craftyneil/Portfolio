use bevy::{asset::Handle, image::Image, prelude::Resource};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct BattAsset {
    #[asset(path = "batt/static_batt.png")]
    pub batt_image: Handle<Image>,
}
