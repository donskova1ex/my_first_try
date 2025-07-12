use bevy::app::{App, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, Commands};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}