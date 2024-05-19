use bevy::prelude::*;
use crate::cameras::{camera_movement, setup_cameras};
use crate::lighting::setup_lighting;
use crate::terrain::setup_terrain;

use crate::player::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_cameras, setup_lighting, setup_terrain));
        app.add_systems(Startup, add_player);
        app.add_systems(Update, camera_movement);
        app.add_systems(Update, player_movement);
    }
}
