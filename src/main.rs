use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy_rapier3d::prelude::*;

mod player;
mod setup;

use player::PlayerPlugin;
use setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SetupPlugin,
            PlayerPlugin,
            WorldInspectorPlugin::new(),
            // RapierDebugRenderPlugin::default()
        ))
        .run();
}
