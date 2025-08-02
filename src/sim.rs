use crate::systems::gravity::*;
use crate::systems::mousescroll::scroll_events;
use crate::systems::spawn::spawn_bodies;
use bevy::prelude::*;
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_bodies))
            // .add_systems(Update, jiggle_bodies)
            .add_systems(FixedUpdate, (scroll_events, apply_gravity, integrate));
    }
}
