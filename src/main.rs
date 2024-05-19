use bevy::prelude::*;

mod cameras;
mod lighting;

mod terrain;
mod setup;
mod player;

mod friend;

use setup::SetupPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, SetupPlugin)).run();
}
