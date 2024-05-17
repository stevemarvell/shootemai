use bevy::prelude::*;
use crate::systems::{setup_camera, setup_lighting, add_player, setup_floor};

pub struct HelloPlugin;

impl Plugin for crate::HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_lighting, setup_floor, add_player ));
    }
}
