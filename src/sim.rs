use crate::systems::sun_behavior::sun_behavior;
use crate::systems::moon_behavior::moon_behavior;
use crate::systems::ghost_follow_camera::ghost_follow_camera;
use crate::systems::spawn::spawn_bodies;
use crate::systems::spawn_ghost::spawn_ghost;
use crate::systems::spawn_sun::spawn_sun;
use crate::systems::spawn_moon::spawn_moon;
use crate::systems::camera::*;
use crate::systems::cursor::*;
use crate::systems::gravity::*;
use crate::systems::keyboardcontrol::camera_movement;
use crate::systems::mousecontrol::{mouse_motion, scroll_events};
use bevy::prelude::*;
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (grab_cursor_on_focus, spawn_bodies, spawn_camera, spawn_ghost, spawn_sun, spawn_moon))
            .add_systems(
                FixedUpdate,
                (
                    grab_cursor_on_focus,
                    camera_movement,
                    mouse_motion,
                    scroll_events,
                    sun_behavior,
                    moon_behavior,
                    ghost_follow_camera,
                    apply_gravity,
                    integrate,
                ),
            );
    }
}
