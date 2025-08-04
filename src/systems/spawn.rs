use crate::components::body::Body;
use noise::{NoiseFn, Perlin};
use rand::{Rng, rng};
use crate::config::*;

use bevy::prelude::*;

pub fn spawn_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let perlin = Perlin::new(42);
    let mesh = meshes.add(Rectangle::default());

    let material = materials.add(Color::srgb(0.9, 0.7, 0.8));

    let count_x = SPAWN_DENSITY;
    let count_y = SPAWN_DENSITY - 2;
    let spacing = 64.0;
    let center_offset = Vec2::new(
        -(count_x as f32 * spacing) / 2.0,
        -(count_y as f32 * spacing) / 2.0,
    );

    for y in 0..count_y {
        for x in 0..count_x {
            let ix = x as f64;
            let iy = y as f64;

            // Base grid position with spacing
            let base_pos = Vec2::new(x as f32 * spacing, y as f32 * spacing) + center_offset;

            // Jitter with Perlin noise
            let jitter_x = perlin.get([ix * 0.2, iy * 0.2]) as f32 * 20.0;
            let jitter_y = perlin.get([iy * 0.2, ix * 0.2]) as f32 * 20.0;

            let position = base_pos + Vec2::new(jitter_x, jitter_y);

            // Velocity from Perlin noise + small randomness
            let vx = perlin.get([ix * 0.3 + 100.0, iy * 0.3 + 100.0]) as f32 * 5.0;
            let vy = perlin.get([iy * 0.3 + 200.0, ix * 0.3 + 200.0]) as f32 * 5.0;

            let mut velocity = Vec2::new(vx, vy);

            // 1 in 10 chance to triple velocity
            if rng().random_ratio(1, 10) {
                velocity *= 13.0;
            }

            spawn_body(
                &mut commands,
                position,
                velocity,
                mesh.clone(),
                material.clone(),
            );
        }
    }
}

fn spawn_body(
    commands: &mut Commands,
    position: Vec2,
    velocity: Vec2,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) {

    let mut radius: f32 = 2.0;
    let mut density = 1.0;
    // 1 in 10 chance to triple velocity
    if rng().random_ratio(1, 100) {
        radius *= 2.3;
    }
    if rng().random_ratio(1, 10) {
     density += 0.1;
    }
    if rng().random_ratio(1, 2) {
        radius += 0.5;   
    }
    if rng().random_ratio(1, 5) {
        radius *= 1.5;
    }
    if rng().random_ratio(1, 100) {
        radius *= 0.5;
    }

    let mass = density * radius.powi(3); // volume ∝ radius³

    commands.spawn((
        Body {
            velocity,
            mass: mass,
            radius: radius,
        },
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform {
            translation: position.extend(0.0),
            scale: Vec3::splat(radius * 2.0),
            ..Default::default()
        },
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
    ));
}
