use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);