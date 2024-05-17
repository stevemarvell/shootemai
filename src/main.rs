use bevy::prelude::*;
mod resources;
mod lighting;
mod plugins;
mod player;
mod terrain;
mod cameras;

use plugins::HelloPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
