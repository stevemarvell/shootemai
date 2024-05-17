use bevy::prelude::{Camera3dBundle, Commands, default, Transform};
use bevy::math::Vec3;

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
