use bevy::prelude::{Color, Commands, Component, Cuboid, default, Mesh, Meshable, ResMut, Sphere, Transform};
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::hierarchy::BuildChildren;

#[derive(Component)]
pub struct Player {
    pub name: String
}

pub fn add_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<StandardMaterial>>,) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Player{ name: "Alpha".to_string() })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.5).mesh().uv(32, 18)),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 1.0, 0.0), // Position the sphere on top of the cube
                ..default()
            });
        });
}

