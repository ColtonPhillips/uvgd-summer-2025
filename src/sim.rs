use crate::systems::camera::spawn_camera;
use crate::systems::gravity::*;
use crate::systems::keyboardcontrol::camera_movement;
use crate::systems::mousecontrol::{mouse_motion, scroll_events};
use crate::systems::spawn::spawn_bodies;
use bevy::prelude::*;
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_bodies, spawn_camera))
            // .add_systems(Update, jiggle_bodies)
            .add_systems(
                FixedUpdate,
                (
                    camera_movement,
                    mouse_motion,
                    scroll_events,
                    apply_gravity,
                    integrate,
                ),
            );
    }
}
