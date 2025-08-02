use bevy::prelude::*;
use bevy::color::palettes::basic::PURPLE;
use crate::components::body::Body;

pub fn spawn_bodies(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Body::default(),
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::from_scale(Vec3::splat(100.0)),
    ));
}
