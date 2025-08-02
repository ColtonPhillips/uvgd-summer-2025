use bevy::prelude::*;

#[derive(Component)]  
pub struct Body {
    pub mass: f32,
    pub radius: f32,
    pub velocity: Vec2,
}

impl Default for Body {
    fn default() -> Self {
        Body {
            mass: 1.0,
            radius: 10.0,
            velocity: Vec2::ZERO
        }
    }
}

