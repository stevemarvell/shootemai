use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod follow;
mod friend;
mod player;
mod setup;

use crate::follow::FollowPlugin;
use friend::FriendPlugin;
use player::PlayerPlugin;
use setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            SetupPlugin,
            PlayerPlugin,
            FriendPlugin,
            FollowPlugin,
        ))
        .run();
}
