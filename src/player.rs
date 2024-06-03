use bevy::hierarchy::BuildChildren;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::setup::cameras::FollowTarget;

const PLAYER_SIZE: f32 = 1.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_turning_movement);
    }
}
#[derive(Component)]
pub struct Player {
    pub name: String,
}

#[derive(Component)]
pub struct Speed(f32);

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let body_colour = Color::BLUE;
    let head_colour = Color::RED;

    commands
        .spawn((
            Name::new("Player"),
            Player {
                name: "Thing One".to_string(),
            },
            Speed(5.0),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE)),
                material: materials.add(StandardMaterial {
                    base_color: body_colour,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 2.5, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    FollowTarget,
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(PLAYER_SIZE / 2.0).mesh().uv(32, 18)),
                        material: materials.add(StandardMaterial {
                            base_color: head_colour,
                            ..default()
                        }),
                        transform: Transform::from_xyz(0.0, PLAYER_SIZE, 0.0), // Position the sphere on top of the cube
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(PbrBundle {
                        mesh: meshes.add(Sphere::new(0.2).mesh().uv(32, 18)),
                        material: materials.add(StandardMaterial {
                            base_color: head_colour,
                            ..default()
                        }),
                        transform: Transform::from_xyz(0.0, 0.0, -PLAYER_SIZE / 2.0),
                        ..default()
                    });
                });
        });
}

pub fn player_turning_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    if let Ok((mut transform, speed)) = player_query.get_single_mut() {
        let forward = transform.forward();

        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation += forward * speed.0 * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation += forward * -speed.0 * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.rotate(Quat::from_rotation_y(speed.0 * time.delta_seconds()));
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.rotate(Quat::from_rotation_y(-speed.0 * time.delta_seconds()));
        }

        let up = transform.up();

        if keyboard_input.pressed(KeyCode::KeyQ) {
            transform.translation += up * speed.0 * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::KeyE) {
            transform.translation -= up * speed.0 * time.delta_seconds();
        }
    }
}
