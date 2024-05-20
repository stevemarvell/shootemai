use bevy::hierarchy::BuildChildren;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::*;

use crate::setup::cameras::FollowMarker;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_movement);
    }
}
#[derive(Component)]
pub struct Player {
    pub name: String,
}

#[derive(Component)]
pub struct Velocity(f32);

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let body_colour = Color::BLUE;
    let head_colour = Color::RED;

    commands
        .spawn((
            Player {
                name: "Thing One".to_string(),
            },
            Velocity(5.0),
            FollowMarker,
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(StandardMaterial {
                    base_color: body_colour,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.5).mesh().uv(32, 18)),
                material: materials.add(StandardMaterial {
                    base_color: head_colour,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 1.0, 0.0), // Position the sphere on top of the cube
                ..default()
            });
        });
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    if let Ok((mut transform, velocity)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * velocity.0 * time.delta_seconds();
        }
    }
}
