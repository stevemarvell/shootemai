use bevy::prelude::*;

mod setup;
mod player;

use setup::SetupPlugin;
use player::PlayerPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, SetupPlugin, PlayerPlugin)).run();
}
