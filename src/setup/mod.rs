use bevy::prelude::*;
use bevy_atmosphere::prelude::AtmospherePlugin;

mod cameras;
mod lighting;
mod terrain;
mod ui;

use lighting::spawn_lighting;
use cameras::CameraPlugin;
use terrain::setup_terrain;
use ui::UiPlugin;


pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, UiPlugin, AtmospherePlugin));
        app.add_systems(Startup, (spawn_lighting, setup_terrain));
    }
}
