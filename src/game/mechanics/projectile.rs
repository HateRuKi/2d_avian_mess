use crate::game::{
    game::{
        GRIDSIZE, GameLayer, GridCoord, GridMap, GridSelectorCoord, GridSelectorPos,
        MAXPLACEDISTANCE, MousePosWorld, Player, ProjectileAttributes, ProjectileCreationEvent,
        SpeedType, TrajectoryType,
    },
    player,
};
use avian2d::{math::*, prelude::*};
use bevy::{prelude::*, sprite::Material2d};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_shoot,
                handle_projectile_creation,
                cleanup_projectiles,
            ),
        )
        .add_event::<ProjectileCreationEvent>();
    }
}

fn player_shoot(
    mut cmd: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    query_player: Query<&Player, With<Player>>,
    mouseposworld: Res<MousePosWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut creation_events: EventWriter<ProjectileCreationEvent>,
) {
    if keys.just_pressed(KeyCode::KeyG) {
        cmd.spawn((
            RigidBody::Dynamic,
            LinearVelocity(Vec2::new(0.0, 100.0)),
            Transform::from_xyz(mouseposworld.x, mouseposworld.y, 0.0),
            Collider::circle(5.0),
            Mesh2d(meshes.add(Circle::new(5.0))),
            MeshMaterial2d(materials.add(Color::srgba_u8(100, 100, 255, 200))),
        ));
        println!("Spawned!");
    }

    if keys.just_pressed(KeyCode::KeyF) {
        println!("Projectile Shot!");

        let player_pos = query_player
            .iter()
            .map(|p| p.position.truncate())
            .next()
            .unwrap_or(Vec2::ZERO);

        creation_events.write(ProjectileCreationEvent {
            entity: Entity::from_raw(0),
            trajectory_type: TrajectoryType::Straight,
            speed_type: SpeedType::Velocity,
            speed: 1000.0,
            direction: Vec2::new(1.0, 1.0), // This might be replaced by actual mouse direction
            origin: player_pos,
        });

        println!("Event Written!");
    }
}

fn handle_projectile_creation(
    mut cmd: Commands,
    mut creation_events: EventReader<ProjectileCreationEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouseposworld: Res<MousePosWorld>,
) {
    for event in creation_events.read() {
        println!("Event Read!");

        let origin = event.origin;
        let mouse_position = Vec2::new(mouseposworld.x, mouseposworld.y);
        let mut direction = mouse_position - origin;

        // Fallback to default direction if zero vector
        if direction.length_squared() == 0.0 {
            direction = Vec2::X;
        } else {
            direction = direction.normalize();
        }

        let angle = direction.y.atan2(direction.x);

        let gravity_scale = match event.trajectory_type {
            TrajectoryType::Projectile => 1.0,
            TrajectoryType::Straight => 0.0,
        };

        let entity = cmd
            .spawn((
                ProjectileAttributes {
                    trajectory_type: event.trajectory_type,
                    speed_type: event.speed_type,
                    speed: event.speed,
                    direction,
                },
                RigidBody::Dynamic,
                Mesh2d(meshes.add(Circle::new(50.0))),
                MeshMaterial2d(materials.add(Color::srgba_u8(100, 100, 255, 200))),
                Collider::circle(50.0),
                CollisionLayers::new(
                    GameLayer::Projectile,
                    [GameLayer::Default, GameLayer::Enemy, GameLayer::Block],
                ),
                GravityScale(gravity_scale),
                Transform {
                    translation: Vec3::new(origin.x, origin.y, 0.0),
                    rotation: Quat::from_rotation_z(angle),
                    ..Default::default()
                },
                LinearVelocity(direction * event.speed),
            ))
            .id();

        println!("Spawned projectile entity: {:?}", entity);
    }
}

fn cleanup_projectiles(
    mut cmd: Commands,
    keys: Res<ButtonInput<KeyCode>>,

    projectiles: Query<(Entity, &Transform), (With<ProjectileAttributes>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<ProjectileAttributes>)>,
) {
    if keys.just_pressed(KeyCode::KeyH) {
        for (entity, projectile) in projectiles.iter() {
            cmd.entity(entity).despawn();
            debug!("Projectile despawned");
        }
    }
    if let Ok(player_transform) = player.single() {
        let player_translation = player_transform.translation;
        //did not detect any projectiles
        for (_, projectile_transform) in projectiles.iter() {
            debug!("hooray!");

            let distance = (player_translation - projectile_transform.translation).length();

            if distance > 1000.0 {
                debug!("Projectile despawned");
            }
        }
    }
}
