mod platform_plugin;
mod player_plugin;
mod level_bounds_plugin;

use level_bounds_plugin::LevelBoundsPlugin;
use platform_plugin::PlatformPlugin;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_rapier2d::prelude::*;
use crate::player_plugin::PlayerPlugin;
use bevy::window::{Window, WindowResolution};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(LevelBoundsPlugin)
        .add_plugins(PlatformPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
