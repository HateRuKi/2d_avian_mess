use avian2d::prelude::*;
use bevy::prelude::*;

use super::super::game::{MousePosWindow, MousePosWorld, Player};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup)
            .add_systems(Update, (camera_movement));
    }
}

fn camera_setup(mut cmd: Commands) {
    println!("Hello From camera.rs!");
    cmd.spawn(Camera2d)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0));
}
fn camera_movement(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouseposworld: Res<MousePosWorld>,
    mouseposwindow: Res<MousePosWindow>,
    window: Query<&Window>,

    query_plauer: Query<(&Transform), (With<Player>, Without<Camera2d>)>,
) {
    //by mouse
    //at the middle of the player and the mouse
    let zoom = 1.0;
    for (player_transform) in query_plauer.iter() {
        let Ok(mut camera_transform) = camera_query.single_mut() else {
            return;
        };
        let Ok(window) = window.single() else {
            return;
        };
        let player_position = player_transform.translation;
        let mouse_position = Vec3::new(mouseposwindow.x, mouseposwindow.y, 0.0);
        let new_position = player_position + (10.0* mouse_position / Vec3::new(window.width(), window.height(), 1.0));
        camera_transform.translation = new_position;
    }
}
