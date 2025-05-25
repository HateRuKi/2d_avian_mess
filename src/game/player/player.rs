use super::super::game::{GameLayer, PLAYERSIZE, Player,JumpSensor};
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
        Mesh2d(meshes.add(Circle::new(PLAYERSIZE))),
        MeshMaterial2d(materials.add(Color::srgba_u8(255, 50, 50, 255))),
        Transform::from_xyz(0.0, 200.0, 0.0),
        Collider::circle(PLAYERSIZE),
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
        AngularDamping(0.2),
        LinearDamping(0.2),
        // LinearDamping(0.2),
        Player {
            acceleration: 100.0,
            damping: 0.2,
            max_speed: 500.0,
            position: Vec3::ZERO,
        },
    ));
    cmd.spawn((
        Sensor,
        Collider::rectangle(PLAYERSIZE * 0.1, PLAYERSIZE * 0.2),
        Transform::from_xyz(0.0, -PLAYERSIZE * 1.0, 0.0),
    ));
}
fn update_player(
    mut player_query: Query<(&Transform, &mut Player), (With<Player>,Without<JumpSensor>)>,
    mut jumpsensor_query: Query<(&mut Transform), (With<JumpSensor>,Without<Player>)>
) {
    for (player_transform, mut player_component) in player_query.iter_mut() {
       let player_translation  =  player_transform.translation;
        player_component.position = player_translation;
        for (mut jumpsensor_transform) in jumpsensor_query.iter_mut() {
            jumpsensor_transform.translation = Vec3::new(player_translation.x, player_translation.y - PLAYERSIZE * 1.0, 0.0);  
        }
    }
}
