use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::PrimaryWindow;

pub struct FollowPlugin;

impl Plugin for FollowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, watch_marker)
            .add_systems(Update, (resize_viewport, toggle_viewer_camera));
    }
}
#[derive(Component)]
pub struct Marker;

#[derive(Component)]
pub struct Watcher;

#[derive(Component)]
pub struct Viewer;

#[derive(Component)]
pub struct ViewerCamera;

fn resize_viewport(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&mut Camera, &Parent), With<ViewerCamera>>,
) {
    let primary_window = window_query.get_single().unwrap();

    for (mut camera, _parent) in camera_query.iter_mut() {
        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(
                (primary_window.width() * 2.0 / 3.0) as u32,
                (primary_window.height() * 2.0 / 3.0) as u32,
            ),
            physical_size: UVec2::new(
                (primary_window.width() / 3.0) as u32,
                (primary_window.height() / 3.0) as u32,
            ),
            ..default()
        });
    }
}

fn toggle_viewer_camera(
    mut commands: Commands,
    viewer_query: Query<(Entity, &Children), With<Viewer>>,
    camera_query: Query<Entity, With<ViewerCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Slash) {
        if let Ok((entity, children)) = viewer_query.get_single() {
            let mut has_camera = false;

            for &child in children.iter() {
                if camera_query.get(child).is_ok() {
                    has_camera = true;
                    // Despawn the existing camera
                    commands.entity(child).despawn_recursive();
                }
            }

            // If there was no camera, spawn a new one
            if !has_camera {
                commands.entity(entity).with_children(|parent| {
                    parent
                        .spawn(Camera3dBundle {
                            camera: Camera {
                                order: 1, // Assign a priority to avoid ambiguities
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, -1.5),
                            ..default()
                        })
                        .insert(ViewerCamera);
                });
            }
        }
    }
}

pub fn watch_marker(
    mut watcher_query: Query<(&mut Transform, &GlobalTransform), With<Watcher>>,
    marker_query: Query<&GlobalTransform, With<Marker>>,
) {
    if let Ok(marker_global_transform) = marker_query.get_single() {
        let global_marker_position = marker_global_transform.translation();

        for (mut watcher_transform, watcher_global_transform) in watcher_query.iter_mut() {
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
