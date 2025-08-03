// src/systems/spawn.rs

use crate::components::body::Body;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::{Rng, rng};

pub fn spawn_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Shouldn't be here
    commands.spawn((
        Camera2d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: (720.0),
            },
            scale: 4.0,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let perlin = Perlin::new(42);
    let mesh = meshes.add(Rectangle::default());

    let material = materials.add(Color::srgb(0.9, 0.7, 0.8));

    let count_x = 48;
    let count_y = 48;
    let spacing = 32.0;
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
    let mut _scale = 10.0;
    let mut _mass = 100.0;
    // 1 in 10 chance to triple velocity
    if rng().random_ratio(1, 100) {
        _mass *= 1.3;
        _scale *= 2.3;
    }

    commands.spawn((
        Body {
            velocity,
            mass: _mass,
            radius: 1.0,
        },
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform {
            translation: position.extend(0.0),
            scale: Vec3::splat(_scale),
            ..Default::default()
        },
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
    ));
}
