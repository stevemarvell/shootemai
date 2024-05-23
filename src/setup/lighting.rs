use crate::setup::WorldOrigin;
use bevy::prelude::*;
use bevy_atmosphere::plugin::AtmospherePlugin;

#[derive(Component)]
struct Sun;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin)
            .add_systems(Startup, spawn_sun)
            .add_systems(Update, animate_sun);
    }
}

fn animate_sun(time: Res<Time>, query: Query<&WorldOrigin>, mut query_sun: Query<&mut Transform, With<Sun>>) {
    if let Ok(world_origin) = query.get_single() {
        let latitude: f32 = world_origin.latitude;
        let latitude_radians = latitude * std::f32::consts::PI / 180.0;

        for mut transform in query_sun.iter_mut() {
            let elapsed_seconds = time.elapsed_seconds() as f32;
            let angle = elapsed_seconds * 0.5; // Adjust the speed as needed

            // Sun's horizontal rotation (east to west)
            let y_rotation = angle;

            // Sun's vertical rotation (up and down, adjusted for latitude)
            let x_rotation = latitude_radians.sin() * (angle.cos() * std::f32::consts::FRAC_PI_4); // Adjust for latitude

            transform.rotation = Quat::from_axis_angle(Vec3::Y, y_rotation) * Quat::from_axis_angle(Vec3::X, x_rotation);
        }
    }
}


pub fn spawn_sun(mut commands: Commands) {
    commands.spawn((
        Sun,
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 1000.0,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::from_rotation(Quat::from_axis_angle(
                Vec3::Y,
                -std::f32::consts::FRAC_PI_2,
            )),
            ..Default::default()
        },
    ));
}
