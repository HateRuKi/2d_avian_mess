use super::super::game::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup))
            .add_systems(Update, (funn));
    }
}
#[derive(Component)]
struct Player;

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
            ],
        ),
        LinearVelocity::default(),
        Player,
    ));
}

fn funn(keys: Res<ButtonInput<KeyCode>>, mut q: Query<(&mut LinearVelocity), With<Player>>) {
    if keys.just_pressed(KeyCode::Space) {
        for (mut lv) in q.iter_mut() {
            println!("Bro Jumped!");
            lv.y = 300.0;
        }
    }
}
