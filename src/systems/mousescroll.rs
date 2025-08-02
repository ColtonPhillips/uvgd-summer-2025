use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
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
                        ortho.scale += 0.15 * -event.y; // Zoom in
                        println!("Zoomed in: scale is now {}", ortho.scale);
                    }
                }
            }
            MouseScrollUnit::Line => {
                for mut projection in &mut query {
                    if let Projection::Orthographic(ortho) = &mut *projection {
                        ortho.scale += 0.39 * -event.y; // Zoom in
                        if ortho.scale < 0.3 {
                            ortho.scale = 0.3;
                        }
                    }
                }
            }
        }
    }
}
