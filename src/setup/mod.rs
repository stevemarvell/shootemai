use bevy::prelude::*;

pub(crate) mod cameras;
mod lighting;
mod terrain;

use lighting::spawn_lighting;
use cameras::CameraPlugin;
use terrain::setup_terrain;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlugin);
        app.add_systems(Startup, (spawn_lighting, setup_terrain));
    }
}
