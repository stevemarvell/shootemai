use bevy::prelude::*;
use crate::cameras::setup_cameras;
use crate::player::add_player;
use crate::lighting::setup_lighting;
use crate::terrain::setup_terrain;

pub struct HelloPlugin;

impl Plugin for crate::HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_cameras, setup_lighting, setup_terrain, add_player ));
    }
}
