use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

pub fn mouse_motion(
    mut events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &Projection), With<Camera>>,
) {
    for event in events.read() {
        for (mut transform, projection) in &mut query {
            if let Projection::Orthographic(ortho) = projection {
                let slow_it_down = 0.65;
                let scale = ortho.scale * slow_it_down;
                transform.translation.x += scale * event.delta.x;
                transform.translation.y -= scale * event.delta.y;
            }
        }
    }
}

// Fine grained control over mouse wheel events
pub fn scroll_events(
    mut events: EventReader<MouseWheel>,
    mut query: Query<&mut Projection, With<Camera>>,
) {
    for event in events.read() {
        match event.unit {
            MouseScrollUnit::Pixel => {
                for mut projection in &mut query {
                    if let Projection::Orthographic(ortho) = &mut *projection {
                        ortho.scale += 0.2 * -event.y; // Zoom in
                        if ortho.scale < 0.2 {
                            ortho.scale = 0.2;
                        }
                    }
                }
            }
            MouseScrollUnit::Line => {
                for mut projection in &mut query {
                    if let Projection::Orthographic(ortho) = &mut *projection {
                        ortho.scale += 0.50 * -event.y; // Zoom in
                        if ortho.scale < 0.5 {
                            ortho.scale = 0.5;
                        }
                    }
                }
            }
        }
    }
}
