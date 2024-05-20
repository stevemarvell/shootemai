use bevy::prelude::*;

mod cameras;
mod lighting;
mod terrain;

use lighting::spawn_lighting;
use cameras::*;
use terrain::setup_terrain;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_lighting, setup_terrain));
        app.add_systems(Update, camera_movement);
    }
}
