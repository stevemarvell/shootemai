use bevy::prelude::*;

mod player;
mod setup;

use player::PlayerPlugin;
use setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SetupPlugin, PlayerPlugin))
        .run();
}
