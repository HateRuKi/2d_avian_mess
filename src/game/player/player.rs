use super::super::game::{GameLayer, Player};
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup))
            .add_systems(Update, (update_player));
    }
}

fn player_setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Hello From player.rs!");

    cmd.spawn((
        RigidBody::Dynamic,
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgba_u8(255, 50, 50, 255))),
        Transform::from_xyz(0.0, 200.0, 0.0),
        Collider::rectangle(50.0, 50.0),
        CollisionLayers::new(
            GameLayer::Player,
            [
                GameLayer::Default,
                GameLayer::Ground,
                GameLayer::Enemy,
                GameLayer::Object,
                GameLayer::Block,
            ],
        ),
        ExternalForce::default(),
        LinearVelocity::default(),
        AngularVelocity::default(),
        // LinearDamping(0.2),
        Player {
            acceleration: 100.0,
            damping: 0.2,
            max_speed: 500.0,
            position: Vec3::ZERO,
        },
    ));
}
fn update_player(mut player_query: Query<(&Transform, &mut Player), With<Player>>) {
    for (player_transform,mut player_component) in player_query.iter_mut() {
        player_component.position = player_transform.translation;
    }
}
