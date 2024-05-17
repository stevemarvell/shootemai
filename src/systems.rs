use crate::components::{Name, Player};
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn setup_lighting(mut commands: Commands) {
    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    });

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 5000.0,  // Lux value, adjust based on your needs
            shadows_enabled: true, // Enable shadows for more realism
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -0.25, // Tilt downward slightly less than before
            0.2,  // Rotate slightly to the right (from the perspective of the light coming from behind)
            0.0   // No roll
        )),
        ..default()
    });
}

pub fn setup_floor(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::SILVER),
        ..default()
    });
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
        .insert(Player)
        .insert(Name("Player1".to_string()))
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
