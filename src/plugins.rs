use bevy::prelude::*;
use crate::resources::GreetTimer;
use crate::systems::{add_players, greet_players, update_players};

pub struct HelloPlugin;

impl Plugin for crate::HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_players)
            .add_systems(Update, (update_players, greet_players).chain());
    }
}
