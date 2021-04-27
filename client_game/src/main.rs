use bevy::prelude::*;
use client_game::{
    camera::CameraPlugin, controls::ControlsPlugin, player::PlayerPlugin, res::ResourcesPlugin,
    scale::ScalePlugin, walls::WallsPlugin,
};

fn main() {
    App::build()
        // Add all default plugins to the app
        .add_plugins(DefaultPlugins)
        .add_plugin(ResourcesPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ControlsPlugin)
        .add_plugin(ScalePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WallsPlugin)
        .run();
}
