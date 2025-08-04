use bevy::prelude::*;
use crate::components::{body::Body, sun::Sun};


pub fn spawn_sun(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_handle = asset_server.load("sprites/sun.png");
    let sprite = Sprite::from_image(image_handle);

    let radius: f32 = 2.0;
    let density = 4.0;

    let mass = density * radius.powi(3); // volume ∝ radius³

    commands.spawn((
        sprite, 
        Sun,
        Body {
            velocity: Vec2 { x: 0.001, y: 0.00002 },
            mass: mass,
            radius: radius,
        },
        Transform {
            translation: Vec3 { x: 1.0, y: 0.1, z: 0.0 },
            scale: Vec3::splat(radius * 2.0 * 0.01),
            ..Default::default()
            },
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
    ));

}

