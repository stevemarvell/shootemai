use bevy::prelude::*;

#[derive(Component)]
pub struct Ui;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_start);
    }
}

pub fn ui_start(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Ui"),
            Ui,
            NodeBundle {
                style: Style {
                    width: Val::Percent(33.0),
                    height: Val::Percent(33.0),
                    left: Val::Percent(67.0),
                    ..default()
                },
                background_color: Color::DARK_GREEN.into(),
                ..default()
            },
        ));
}