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
pub struct WatchMarker;

pub fn watch_marker(
    mut watcher_query: Query<&mut Transform, With<Watcher>>,
    follow_marker_query: Query<(&GlobalTransform, &Parent), With<WatchMarker>>,
) {
    if let Ok((marker_global_transform, _)) = follow_marker_query.get_single() {
        let global_marker_position = marker_global_transform.translation();

        for mut watcher_transform in watcher_query.iter_mut() {
            watcher_transform.look_at(global_marker_position, Vec3::Y);
        }
    }
}

/*
pub fn watch_marker(
    mut watcher_query: Query<&mut Transform, With<Watcher>>,
    follow_marker_query: Query<&Transform, (With<WatchMarker>, Without<Watcher>)>,
) {
    if let Ok(follow_marker_transform) = follow_marker_query.get_single() {
        for mut watcher_transform in watcher_query.iter_mut() {

            watcher_transform.look_at(follow_marker_transform.translation, Vec3::Y);
        }
    }
}
*/