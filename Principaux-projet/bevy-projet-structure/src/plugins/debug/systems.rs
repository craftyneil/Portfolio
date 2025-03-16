use bevy::prelude::*;
use super::ressource::AssetsLoading;

pub(super) fn setup(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    // we can have different asset types
    let batt: Handle<Image> = server.load("batt/static_batt.png");

    // add them all to our collection for tracking
    loading.0.push(batt.untyped());
}

pub(super) fn check_assets_ready(
    //mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>
) {
    use bevy::asset::LoadState;

    for asset in &loading.0 {
        match server.get_load_state(asset.id()) {
            Some(LoadState::Failed(err)) => {
                // one of our assets had an error
                error!("asset error: {}", err);
            }
            Some(LoadState::Loaded) => {
                // all assets are now ready
                info!("Everything is ok");
                // this might be a good place to transition into your in-game state

                // remove the resource to drop the tracking handles
                //commands.remove_resource::<AssetsLoading>();
                // (note: if you don't have any other handles to the assets
                // elsewhere, they will get unloaded after this)
            }
            _ => {
                // NotLoaded/Loading: not fully ready yet
            }
        }
    }
}