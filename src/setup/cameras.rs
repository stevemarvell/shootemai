use crate::follow::Watcher;
use bevy::input::ButtonInput;
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

const CAMERA_SPEED: f32 = 1.0;

pub fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(mut transform) = camera_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyQ) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        transform.translation += direction * CAMERA_SPEED * time.delta_seconds();

        // pan and tilt

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.rotate(Quat::from_rotation_y(CAMERA_SPEED * time.delta_seconds()));
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.rotate(Quat::from_rotation_y(-CAMERA_SPEED * time.delta_seconds()));
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.rotate_local(Quat::from_rotation_x(-CAMERA_SPEED * time.delta_seconds()));
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.rotate_local(Quat::from_rotation_x(CAMERA_SPEED * time.delta_seconds()));
        }
    }
}
