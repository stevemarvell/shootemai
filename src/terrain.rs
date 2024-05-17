use bevy::prelude::{Color, Commands, default, Mesh, Meshable, Plane3d, ResMut};
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, StandardMaterial};

pub fn setup_terrain(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::SILVER),
        ..default()
    });
}
