use bevy::prelude::*;

mod plugins;
fn main() {
    App::new().add_plugins(plugins::BevyPlugin).run();
}
