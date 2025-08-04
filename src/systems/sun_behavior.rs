use bevy::prelude::*;
use crate::components::sun::Sun;
use crate::components::body::Body;

pub fn sun_behavior(
    mut sun_q: Query<(&Transform, &mut Body), With<Sun>>,
    all_bodies_q: Query<(&Transform, &Body), (With<Body>, Without<Sun>)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    let Ok((sun_transform, mut sun_body)) = sun_q.single_mut() else { return; };

    let mut weighted_sum = Vec2::ZERO;
    let mut total_mass = 0.0;

    for (transform, body) in all_bodies_q.iter() {
        let mass = body.mass;
        let pos = Vec2::new(transform.translation.x, transform.translation.y);
        weighted_sum += pos * mass;
        total_mass += mass;
    }

    if total_mass == 0.0 {
        sun_body.velocity = Vec2::ZERO;
        return;
    }

    let center_of_mass = weighted_sum / total_mass;
    let sun_pos = Vec2::new(sun_transform.translation.x, sun_transform.translation.y);
    let dir = center_of_mass - sun_pos;

    if dir.length_squared() > 3.0 {
        let speed = 1.0;
        sun_body.velocity += dir.normalize() * speed * dt;
    } else {
        sun_body.velocity = Vec2::ZERO;
    }
}
