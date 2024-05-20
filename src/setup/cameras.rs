use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::math::Vec3;

pub fn spawn_camera(mut commands: Commands) {

    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(camera);
}

const CAMERA_SPEED:f32 = 1.0;

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