use bevy::math::Vec3;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_follow_camera);
        app.add_systems(Update, follow_camera_system);
    }
}

#[derive(Component)]
struct FollowCamera;


#[derive(Component)]
pub struct FollowTarget;

fn follow_camera_system(
    mut follow_camera_query: Query<(&mut Transform, &GlobalTransform), With<FollowCamera>>,
    marker_query: Query<&GlobalTransform, With<FollowTarget>>,
) {
    if let Ok(marker_global_transform) = marker_query.get_single() {
        let global_marker_position = marker_global_transform.translation();

        for (mut watcher_transform, watcher_global_transform) in follow_camera_query.iter_mut() {
            let global_watcher_position = watcher_global_transform.translation();
            let direction_to_marker =
                (global_marker_position - global_watcher_position).normalize();

            // Calculate the right and up vectors to form an orthogonal basis
            let right = direction_to_marker.cross(Vec3::Y).normalize();
            let up = right.cross(direction_to_marker).normalize();

            // Create a rotation matrix from these vectors
            let rotation_matrix = Mat3::from_cols(right, up, -direction_to_marker);

            // Convert the rotation matrix to a quaternion
            let rotation_quat = Quat::from_mat3(&rotation_matrix);

            // Set the watcher's rotation
            watcher_transform.rotation = rotation_quat;
        }
    }
}

pub fn spawn_follow_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FollowCamera,
        IsDefaultUiCamera,
    ));
}
