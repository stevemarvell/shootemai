use bevy::prelude::*;
use bevy_atmosphere::plugin::AtmospherePlugin;

#[derive(Component)]
struct Sun;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AtmospherePlugin)
            .add_systems(Startup, spawn_sun)
            .add_systems(Update, animate_sun);
    }
}

fn animate_sun(time: Res<Time>, mut query: Query<&mut Transform, With<Sun>>) {
    for mut transform in query.iter_mut() {
        let elapsed_seconds = time.elapsed_seconds() as f32;
        let angle = elapsed_seconds * 0.5; // Adjust the speed as needed

        let x_rotation = -angle.cos() * std::f32::consts::FRAC_PI_4; // -45 to 45 degrees
        let y_rotation = angle;

        transform.rotation = Quat::from_rotation_y(y_rotation) * Quat::from_rotation_x(x_rotation);
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
            transform: Transform::from_rotation(Quat::from_rotation_y(
                -std::f32::consts::FRAC_PI_2, // Initial rotation to face east
            )),
            ..Default::default()
        },
    ));
}
