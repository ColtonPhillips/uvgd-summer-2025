use bevy::prelude::*;
use crate::components::ghost::Ghost;
use crate::components::body::Body; 

pub fn ghost_follow_camera(
    camera_q: Query<&Transform, With<Camera>>,
    mut ghost_q: Query<(&Transform, &mut Body), With<Ghost>>,
    time: Res<Time>
) {
    let dt = time.delta_secs();

    let Ok(camera_transform) = camera_q.single() else { return; };
    let Ok((ghost_transform, mut body)) = ghost_q.single_mut() else { return; };

    let dir = camera_transform.translation - ghost_transform.translation;
    let dir_2d = Vec2::new(dir.x, dir.y); // ignore Z

    if dir_2d.length_squared() > 1.0 {
        let speed = 9.0;
        body.velocity += dir_2d.normalize() * speed * dt;
    } else {
        // Optionally slow down or stop
        body.velocity = Vec2::ZERO;
    }
}
