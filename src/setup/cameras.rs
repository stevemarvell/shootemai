use crate::follow::Watcher;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_atmosphere::prelude::AtmosphereCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    let camera = (
        Name::new("Main Camera"),
        Camera3dBundle {
            camera: Camera {
                order: 0, // Assign a priority to avoid ambiguities
                ..default()
            },
            transform: Transform::from_xyz(0.0, 2.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        IsDefaultUiCamera,
        AtmosphereCamera::default(),
        Watcher,
    );

    commands.spawn(camera);
}