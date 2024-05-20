use bevy::prelude::*;

mod setup;
mod friend;
mod player;

use setup::SetupPlugin;
use player::PlayerPlugin;
use friend::FriendPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SetupPlugin, PlayerPlugin, FriendPlugin))
        .run();
}
