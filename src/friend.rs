use crate::follow::*;
use bevy::hierarchy::BuildChildren;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::*;

pub struct FriendPlugin;

impl Plugin for FriendPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_friend);
    }
}

#[derive(Component)]
pub struct Friend {
    pub name: String,
}

pub fn spawn_friend(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let body_colour = Color::YELLOW;
    let head_colour = Color::RED;

    commands
        .spawn((
            Friend {
                name: "Thing Two".to_string(),
            },
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(StandardMaterial {
                    base_color: body_colour,
                    ..default()
                }),
                transform: Transform::from_xyz(3.0, 0.5, 0.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Watcher,
                    Viewer,
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(0.5).mesh().uv(32, 18)),
                        material: materials.add(StandardMaterial {
                            base_color: head_colour,
                            ..default()
                        }),
                        transform: Transform::from_xyz(0.0, 1.0, 0.0), // Position the sphere on top of the cube
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
                        transform: Transform::from_xyz(0.0, 0.0, -0.5),
                        ..default()
                    });
                });
        });
}