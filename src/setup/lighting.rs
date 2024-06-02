use crate::setup::WorldOrigin;
use bevy::prelude::*;
use bevy_atmosphere::plugin::AtmospherePlugin;
use chrono::Datelike;

#[derive(Component)]
struct Sun;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin)
            .add_systems(Startup, spawn_sun);
//           .add_systems(Update, (sun_rise_and_set, blue_and_golden_hours));
    }
}

fn sun_rise_and_set(time: Res<Time>, query: Query<&WorldOrigin>, mut query_sun: Query<&mut Transform, With<Sun>>) {
    if let Ok(world_origin) = query.get_single() {
        let latitude: f32 = world_origin.latitude;
        let latitude_radians = latitude * std::f32::consts::PI / 180.0;

        // Calculate the day of the year
        // let day_of_year = world_origin.date.ordinal() as f32;

        let day_of_year = world_origin.date.ordinal() as f32;

        // Calculate the declination angle based on the day of the year
        // Simplified formula: declination = 23.44 * cos((360 / 365) * (day_of_year + 10))
        let declination = 23.44 * (360.0 / 365.0 * (day_of_year + 10.0)).to_radians().cos();
        let declination_radians = declination * std::f32::consts::PI / 180.0;

        if let Ok(mut transform) = query_sun.get_single_mut() {
            let elapsed_seconds = time.elapsed_seconds() as f32;
            let angle = elapsed_seconds * 0.5; // Adjust the speed as needed

            // Sun's horizontal rotation (east to west)
            let y_rotation = angle;

            // Sun's vertical rotation (up and down, adjusted for latitude and declination)
            let x_rotation = latitude_radians.sin() * declination_radians.cos() * (angle.cos() * std::f32::consts::FRAC_PI_4); // Adjust for latitude and declination

            transform.rotation = Quat::from_axis_angle(Vec3::Y, y_rotation) * Quat::from_axis_angle(Vec3::X, x_rotation);
        }
    }
}

fn blue_and_golden_hours(time: Res<Time>,  mut query_sun: Query<&mut DirectionalLight, With<Sun>>) {

    let elapsed_seconds = time.elapsed_seconds() as f32;

    let time_of_day = elapsed_seconds * 0.5; // time of day in hours

    // Define golden hour and blue hour periods (in hours)
    let golden_hour_start = 6.0;
    let golden_hour_end = 8.0;
    let blue_hour_start = 18.0;
    let blue_hour_end = 20.0;

    if let Ok(mut light) = query_sun.get_single_mut() {
        if (golden_hour_start <= time_of_day && time_of_day <= golden_hour_end)
            || (blue_hour_start <= time_of_day && time_of_day <= blue_hour_end) {
            // Golden hour or blue hour adjustments
            if golden_hour_start <= time_of_day && time_of_day <= golden_hour_end {
                // Golden hour: warm light
                light.color = Color::rgb(1.0, 0.7, 0.3);
                light.illuminance = light_consts::lux::CLEAR_SUNRISE
            } else {
                // Blue hour: cool light
                light.color = Color::rgb(0.3, 0.5, 1.0);
                light.illuminance = light_consts::lux::CIVIL_TWILIGHT;
            }
        } else {
            // Regular daylight
            light.color = Color::rgb(1.0, 1.0, 1.0);
            light.illuminance = light_consts::lux::AMBIENT_DAYLIGHT;
        }
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
                -std::f32::consts::FRAC_PI_2 / 4.0,
            )),
            ..Default::default()
        },
    ));
}
