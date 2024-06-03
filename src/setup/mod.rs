use bevy::prelude::*;

pub(crate) mod cameras;
mod lighting;
mod terrain;
mod ui;

use lighting::LightingPlugin;
use cameras::CameraPlugin;
use terrain::setup_terrain;
use ui::UiPlugin;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, LightingPlugin, CameraPlugin));
        app.add_systems(Startup, setup_terrain);
    }
}
