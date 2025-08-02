mod setup;
mod components;
mod systems;
use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;

use crate::setup::*;
use crate::systems::debug::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::Immediate,
                    ..default()
                }),
                ..default()
            }),
            FpsOverlayPlugin::default()
        ))
        .add_systems(Startup, (hello_world, spawn_bodies))
        //.add_systems(Update, (jiggle_bodies, log_bodies))
        .add_systems(Update, (jiggle_bodies))
        .run();
}

fn hello_world() {
    info!("I am here");
}

