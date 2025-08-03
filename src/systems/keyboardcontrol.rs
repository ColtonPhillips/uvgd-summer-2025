use bevy::prelude::*;
pub fn camera_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Projection), With<Camera2d>>,
) {
    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }

    let mut speed = 5.0;
    for (mut transform, projection) in &mut query {
        if let Projection::Orthographic(ortho) = projection {
            speed *= ortho.scale;
        }
        transform.translation += direction * speed;
    }
}
