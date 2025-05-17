use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use super::super::game::GameLayer;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_level);
    }
}

fn init_level(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Hello From level.rs!");
    cmd.spawn((
        RigidBody::Static,
        Mesh2d(meshes.add(Rectangle::new(200.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgba_u8(10, 100, 10, 255))),
        Transform::from_xyz(0.0, -50.0, 0.0),
        Collider::rectangle(200.0, 50.0),
        CollisionLayers::new(
            GameLayer::Ground,
            [
                GameLayer::Default,
                GameLayer::Player,
                GameLayer::Enemy,
                GameLayer::Object,
            ],
        ),
    ));
}
