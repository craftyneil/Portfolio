use bevy::prelude::*;

mod general_component;
mod player;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
