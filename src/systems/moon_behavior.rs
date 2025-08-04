use bevy::prelude::*;
use crate::components::moon::Moon;
use crate::components::sun::Sun;
use crate::components::body::Body;

pub fn moon_behavior(
    mut moon_q: Query<(&Transform, &mut Body), (With<Moon>, Without<Sun>)>,
    sun_q: Query<(&Transform, &Body), With<Sun>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    let Ok((moon_transform, mut moon_body)) = moon_q.single_mut() else { return; };
    let Ok((sun_transform, sun_body)) = sun_q.single() else { return; };

    let moon_pos = Vec2::new(moon_transform.translation.x, moon_transform.translation.y);
    let sun_pos = Vec2::new(sun_transform.translation.x, sun_transform.translation.y);
    let to_sun = sun_pos - moon_pos;

    let distance_squared = to_sun.length_squared();
    if distance_squared < 0.001 {
        return;
    }

    let direction = to_sun.normalize();

    // Fake gravitational constant (tune this)
    let gravitational_constant = 200.0;

    // F = G * M / r^2
    let force = direction * gravitational_constant * sun_body.mass / distance_squared;

    // a = F / m (assuming moon mass = 1.0)
    let acceleration = force;

    moon_body.velocity += acceleration * dt;
}
