use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

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
            SetupPlugin,
            PlayerPlugin,
            FriendPlugin,
            FollowPlugin,
            WorldInspectorPlugin::new(),
            RapierDebugRenderPlugin::default()
        ))
        .run();
}
