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
    // Define RGBA colors for grass
    let fresh_spring_grass = Color::Rgba {
        red: 124.0 / 255.0,
        green: 252.0 / 255.0,
        blue: 0.0 / 255.0,
        alpha: 1.0,
    };
    let healthy_summer_grass = Color::Rgba {
        red: 34.0 / 255.0,
        green: 139.0 / 255.0,
        blue: 34.0 / 255.0,
        alpha: 1.0,
    };
    let standard_lawn_grass = Color::Rgba {
        red: 85.0 / 255.0,
        green: 107.0 / 255.0,
        blue: 47.0 / 255.0,
        alpha: 1.0,
    };
    let dry_autumn_grass = Color::Rgba {
        red: 189.0 / 255.0,
        green: 183.0 / 255.0,
        blue: 107.0 / 255.0,
        alpha: 1.0,
    };
    let dark_lush_grass = Color::Rgba {
        red: 0.0 / 255.0,
        green: 100.0 / 255.0,
        blue: 0.0 / 255.0,
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
