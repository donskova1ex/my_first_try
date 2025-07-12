mod platform_plugin;
mod player_plugin;
use platform_plugin::PlatformPlugin;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::player_plugin::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PlatformPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
