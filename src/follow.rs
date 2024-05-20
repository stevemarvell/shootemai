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
    mut watcher_query: Query<&mut Transform, With<Watcher>>,
    marker: Query<&GlobalTransform, With<Marker>>,
) {
    if let Ok(marker_global_transform) = marker.get_single() {
        let global_marker_position = marker_global_transform.translation();

        for mut watcher_transform in watcher_query.iter_mut() {
            watcher_transform.look_at(global_marker_position, Vec3::Y);
        }
    }
}