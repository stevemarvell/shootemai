use bevy::prelude::*;
use crate::components::{Player, Name};
use crate::resources::GreetTimer;

pub fn add_players(mut commands: Commands) {
    commands.spawn((Player, Name("Elaina Proctor".to_string())));
    commands.spawn((Player, Name("Renzo Hume".to_string())));
    commands.spawn((Player, Name("Zayna Nieves".to_string())));
}

pub fn greet_players(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Player>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub fn update_players(mut query: Query<&mut Name, With<Player>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}
