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
                illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::from_rotation(Quat::from_axis_angle(
                Vec3::Y,
                std::f32::consts::FRAC_PI_2 / 3.0,
            )),
            ..Default::default()
        },
    ));
}
