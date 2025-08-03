use bevy::prelude::*;
use crate::components::{body::Body, ghost::Ghost};


pub fn spawn_ghost(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_handle = asset_server.load("sprites/ghost.png");
    let sprite = Sprite::from_image(image_handle);

    let radius: f32 = 0.3;
    let density = 0.6;

    let mass = density * radius.powi(3); // volume ∝ radius³

    commands.spawn((
        sprite, 
        Ghost,
        Body {
            velocity: Vec2 { x: 0.0, y: 0.0 },
            mass: mass,
            radius: radius,
        },
        Transform {
            translation: Vec3 { x: 10.0, y: 1.0, z: 0.0 },
            scale: Vec3::splat(radius * 2.0),
            ..Default::default()
            },
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
    ));

}

