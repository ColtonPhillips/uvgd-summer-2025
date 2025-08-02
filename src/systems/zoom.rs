// use bevy::prelude::*;

// pub fn zoom_with_key(keys: Res<Input<KeyCode>>, mut query: Query<&mut Projection, With<Camera>>) {
//     if keys.just_pressed(KeyCode::Z) {
//         for mut projection in &mut query {
//             if let Projection::Orthographic(ortho) = &mut *projection {
//                 ortho.scale *= 0.5; // Zoom in
//                 println!("Zoomed in: scale is now {}", ortho.scale);
//             }
//         }
//     }
// }
