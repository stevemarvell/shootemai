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
    mut follow_camera_query: Query<&mut Transform, With<FollowCamera>>,
    target_query: Query<&GlobalTransform, With<FollowTarget>>,
) {
    if let Ok(target_global_transform) = target_query.get_single() {
        let global_target_position = target_global_transform.translation();

        for mut follower_transform in follow_camera_query.iter_mut() {

            // Define the offset for the camera to follow from behind
            let follow_distance = 10.0;
            let follow_height = 5.0;

            // Adjust the offset to ensure the camera follows from behind the target
            let offset = -target_global_transform.forward() * follow_distance + Vec3::Y * follow_height;
            let new_camera_position = global_target_position + offset;

            // Update the camera's position
            follower_transform.translation = new_camera_position;

            // Make the camera look at the target
            follower_transform.look_at(global_target_position, Vec3::Y);
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
