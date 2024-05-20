use bevy::prelude::*;

pub fn spawn_lighting(mut commands: Commands) {

    // Ambient light

    let ambient_light = AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
    };

    // Directional light

    let directional_light = DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -0.25,
            0.2,
            0.0
        )),
        ..default()
    };

    commands.insert_resource(ambient_light);
    commands.spawn(directional_light);
}
