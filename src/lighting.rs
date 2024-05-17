use bevy::prelude::*;

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
