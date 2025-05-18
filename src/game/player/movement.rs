use super::super::game::{GameLayer, Player};
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (horizontal_movement, jump));
    }
}

fn horizontal_movement(

    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut ExternalForce, &mut LinearVelocity, &Player), With<Player>>,
    time: Res<Time>,
) {
    let speed = 500.0;
    for (mut force, mut linear_velocity, player) in q.iter_mut() {
        // println!("Hello From player_movement.rs!");
        let acceleration = player.acceleration;
        let max_speed = player.max_speed;
        let speed = 500.0;

        //Plan A
        if keys.pressed(KeyCode::KeyA) {
            linear_velocity.x -= acceleration * 10.0 * time.delta_secs();
        }

        if keys.pressed(KeyCode::KeyD) {
            linear_velocity.x += acceleration * 10.0 * time.delta_secs();
        }

        // //Plan B
        // if keys.pressed(KeyCode::KeyA) {
        //     linear_velocity.x = speed * time.delta_secs();
        // }

        // if keys.pressed(KeyCode::KeyD) {
        //     linear_velocity.x = speed * time.delta_secs();
        // }

        linear_velocity.x = linear_velocity.x.clamp(-max_speed, max_speed);

        println!("Current velocity: {:?}", linear_velocity,);
    }
}
fn jump(keys: Res<ButtonInput<KeyCode>>, mut q: Query<(&mut LinearVelocity), With<Player>>) {
    for (mut linear_velocity) in q.iter_mut() {
        if keys.just_pressed(KeyCode::Space) {
            println!(
                "Jump impulse applied! Current velocity: {:?}",
                linear_velocity
            );
            println!("Bro Jumped!");
            linear_velocity.y = 500.0;
        }
    }
}
