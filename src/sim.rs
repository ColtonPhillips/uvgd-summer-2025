use crate::setup::*;
use crate::systems::debug::*;
use crate::systems::gravity::*;
use crate::systems::spawn::spawn_bodies;
use bevy::prelude::*;
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (hello_world, spawn_bodies))
            // .add_systems(Update, jiggle_bodies)
            .add_systems(FixedUpdate, (apply_gravity, integrate));
    }
}
