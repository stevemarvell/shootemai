use bevy::math::Vec3;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraState::default());
        app.add_systems(Startup, spawn_follow_camera);
        app.add_systems(Update, (follow_camera_system, orbit_camera_system));
    }
}

#[derive(Component)]
struct FollowCamera;

#[derive(Component)]
pub struct FollowTarget;

#[derive(Resource)]
struct CameraState {
    pub distance: f32,
    pub vertical_angle: f32,
    pub horizontal_angle: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        CameraState {
            distance: 10.0,
            vertical_angle: std::f32::consts::FRAC_PI_8,
            horizontal_angle: 3.0 * std::f32::consts::FRAC_PI_4,
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

fn follow_camera_system(
    mut follow_camera_query: Query<&mut Transform, With<FollowCamera>>,
    target_query: Query<&GlobalTransform, With<FollowTarget>>,
    camera_state: Res<CameraState>,
) {
    if let Ok(target_global_transform) = target_query.get_single() {
        let global_target_position = target_global_transform.translation();

        for mut follow_camera_transform in follow_camera_query.iter_mut() {
            // Calculate the offset based on the current angles and distance from the state
            let offset = Vec3::new(
                camera_state.distance
                    * camera_state.horizontal_angle.cos()
                    * camera_state.vertical_angle.cos(),
                camera_state.distance * camera_state.vertical_angle.sin(),
                camera_state.distance
                    * camera_state.horizontal_angle.sin()
                    * camera_state.vertical_angle.cos(),
            );

            // Apply the target's rotation to the offset
            let rotated_offset = target_global_transform.compute_transform().rotation * offset;

            let new_camera_position = global_target_position + rotated_offset;

            // Update the camera's position
            follow_camera_transform.translation = new_camera_position;

            // Make the camera look at the target
            follow_camera_transform.look_at(global_target_position, Vec3::Y);
        }
    }
}

fn orbit_camera_system(
    mut camera_state: ResMut<CameraState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    camera_state.vertical_angle = camera_state.vertical_angle.clamp(
        -std::f32::consts::FRAC_PI_2 + 0.1,
        std::f32::consts::FRAC_PI_2 - 0.1,
    );

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        camera_state.horizontal_angle += 1.0 * delta_time;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        camera_state.horizontal_angle -= 1.0 * delta_time;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        camera_state.vertical_angle += 1.0 * delta_time;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        camera_state.vertical_angle -= 1.0 * delta_time;
    }
    if keyboard_input.pressed(KeyCode::PageUp) {
        camera_state.distance += 1.0 * delta_time;
    }
    if keyboard_input.pressed(KeyCode::PageDown) {
        camera_state.distance -= 1.0 * delta_time;
    }
}
