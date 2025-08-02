mod components;
mod setup;
mod sim;
mod systems;
use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;

use sim::SimulationPlugin;

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
            FpsOverlayPlugin::default(),
        ))
        // set the global default clear color
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.2)))
        .add_plugins(SimulationPlugin)
        .run();
}
