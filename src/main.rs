use bevy::prelude::*;
mod lighting;
mod setup;
mod player;
mod terrain;
mod cameras;

use setup::SetupPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, SetupPlugin)).run();
}
