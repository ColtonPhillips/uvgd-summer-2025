use bevy::prelude::*;
use rand::Rng;
use crate::components::body::Body;

pub fn log_bodies(query: Query<&Body>) {
    for body in query.iter() {
        info!("Body mass: {}, velocity: {:?}", body.mass, body.velocity);
    }
}

pub fn jiggle_bodies(mut query: Query<&mut Transform, With<Body>>) {
    let mut rng = rand::rng();

    for mut transform in query.iter_mut() {
        let dx = rng.random_range(-0.5..=0.5);
        let dy = rng.random_range(-0.5..=0.5);
        transform.translation.x += dx;
        transform.translation.y += dy;
    }
}

