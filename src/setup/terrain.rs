use bevy::asset::Assets;
use bevy::core::Name;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_terrain(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let healthy_summer_grass = Color::Rgba {
        red: 34.0 / 255.0,
        green: 139.0 / 255.0,
        blue: 34.0 / 255.0,
        alpha: 1.0,
    };

    let floor = (
        Name::new("Floor"),
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
            material: materials.add(StandardMaterial {
                base_color: healthy_summer_grass,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(25.0, 0.1, 25.0),
    );

    commands.spawn(floor);
}
