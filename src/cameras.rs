use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::math::Vec3;

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

const CAMERA_SPEED:f32 = 10.0;

pub fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
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
    }
}