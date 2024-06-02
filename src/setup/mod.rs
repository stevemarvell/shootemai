use std::process::Command;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use chrono::{NaiveDate};

mod cameras;
mod lighting;
mod terrain;
mod ui;

use lighting::LightingPlugin;
use cameras::CameraPlugin;
use terrain::setup_terrain;
use ui::UiPlugin;

#[derive(Component)]
struct WorldOrigin {
    latitude: f32,
    longitude: f32,
    date: NaiveDate,
}
fn setup_world(mut commands: Commands) {
    commands.spawn(WorldOrigin {
        latitude: 54.0, // Example latitude
        longitude: 0.0, // Example longitude
        // @TODO advance data with each midnight
        date: NaiveDate::from_ymd_opt(2024, 5, 24).unwrap(),
    });
}
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, UiPlugin, LightingPlugin));
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_systems(Startup, (setup_world, setup_terrain));
    }
}
