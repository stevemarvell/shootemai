use bevy::prelude::*;

#[derive(Component)]
struct Sun;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sun);
    }
}

pub fn spawn_sun(mut commands: Commands) {
    commands.spawn((
        Sun,
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: light_consts::lux::OVERCAST_DAY,
                shadows_enabled: true,
                ..Default::default()
            },
            // Adjust the rotation to simulate a 10 AM sun position
            transform: Transform::from_rotation(Quat::from_axis_angle(
                Vec3::X,
                -std::f32::consts::FRAC_PI_4, // 45 degrees above the horizon
            )),
            ..Default::default()
        },
    ));
}
