use bevy::prelude::*;
use bevy::math::Vec3;

pub struct FollowPlugin;

impl Plugin for FollowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, watch_marker);
    }
}
#[derive(Component)]
pub struct Watcher;
#[derive(Component)]
pub struct Marker;

pub fn watch_marker(
    mut watcher_query: Query<(&mut Transform, &GlobalTransform), With<Watcher>>,
    marker_query: Query<&GlobalTransform, With<Marker>>,
) {
    if let Ok(marker_global_transform) = marker_query.get_single() {
        let global_marker_position = marker_global_transform.translation();

        for (mut watcher_transform, watcher_global_transform) in watcher_query.iter_mut() {
            let global_watcher_position = watcher_global_transform.translation();
            let direction_to_marker = (global_marker_position - global_watcher_position).normalize();

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
