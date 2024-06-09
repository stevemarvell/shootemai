use bevy::prelude::*;

pub(crate) mod cameras;
mod lighting;
mod terrain;

use lighting::LightingPlugin;
use cameras::CameraPlugin;
use terrain::TerrainPlugin;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TerrainPlugin, LightingPlugin, CameraPlugin));
    }
}
