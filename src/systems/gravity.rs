use crate::components::body::Body;
use bevy::prelude::*;

/// Compute gravitational force from `a` to `b`
fn gravitational_force(a: &Transform, am: f32, b: &Transform, bm: f32) -> Vec2 {
    // const G: f32 = 1000.0;
    // const G: f32 = 25.0;
    // const G: f32 = 225.0;
    const G: f32 = 425.0;
    let pos_a = a.translation.truncate();
    let pos_b = b.translation.truncate();
    let dir = pos_b - pos_a;
    let dist_sq = dir.length_squared().max(25.0); // Prevent blowup at short range
    let force_dir = dir.normalize();
    let force_mag = G * am * bm / dist_sq;
    force_dir * force_mag
}

pub fn apply_gravity(mut query: Query<(&mut Body, &Transform)>, time: Res<Time>) {
    let dt = time.delta_secs();

    // Clone the query so we can access all bodies in a second pass
    let bodies: Vec<_> = query.iter().collect();
    let mut forces = vec![Vec2::ZERO; bodies.len()];

    // Only compute each pair once (i < j)
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            let (body_a, transform_a) = bodies[i];
            let (body_b, transform_b) = bodies[j];

            let force = gravitational_force(transform_a, body_a.mass, transform_b, body_b.mass);

            // Newton's third law: equal and opposite
            forces[i] += force / body_a.mass;
            forces[j] -= force / body_b.mass;
        }
    }

    // Apply accumulated accelerations as velocity updates
    for ((mut body, _), acc) in query.iter_mut().zip(forces) {
        body.velocity += acc * dt;
    }
}

pub fn integrate(mut query: Query<(&mut Transform, &Body)>) {
    for (mut transform, body) in &mut query {
        let delta = body.velocity * 0.016; // 60fps
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }
}
