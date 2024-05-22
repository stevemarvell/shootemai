use bevy::asset::Assets;
use bevy::core::Name;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{default, Color, Commands, Mesh, Meshable, Plane3d, ResMut};

pub fn setup_terrain(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let floor = (Name::new("Floor"),PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    commands.spawn(floor);
}
