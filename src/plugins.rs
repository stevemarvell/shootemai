use bevy::prelude::*;
use crate::resources::GreetTimer;
use crate::systems::{add_people, greet_people, update_people};

pub struct HelloPlugin;

impl Plugin for crate::HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (update_people, greet_people).chain());
    }
}
