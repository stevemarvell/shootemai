use bevy::prelude::*;
mod components;
mod resources;
mod systems;
mod plugins;

use plugins::HelloPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
