use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowFocused};

pub fn grab_cursor_on_focus(
    mut focus_events: EventReader<WindowFocused>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    for event in focus_events.read() {
        if event.focused {
            if let Ok(mut window) = windows.single_mut() {
                window.cursor_options.visible = false;
                window.cursor_options.grab_mode = CursorGrabMode::Locked;

                let size = window.resolution.physical_size();
                let center = Vec2::new(size.x as f32 / 2.0, size.y as f32 / 2.0);
                let _ = window.set_cursor_position(Some(center));
            }
        }
        else {
            if let Ok(mut window) = windows.single_mut() {
                window.cursor_options.visible = true;
                window.cursor_options.grab_mode = CursorGrabMode::None;
            }
        }
    }
}


