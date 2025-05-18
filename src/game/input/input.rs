use super::super::game::{GameLayer, MousePosWindow, MousePosWorld, Player};
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_mouse,));
    }
}

fn handle_mouse(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
    mut mousepos: ResMut<MousePosWorld>,
    mut mouseposwindow: ResMut<MousePosWindow>,
) {
    let (camera, camera_transform) = *camera_query;
    let Ok(window) = window.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };
    (mouseposwindow.x, mouseposwindow.y) = (
        cursor_position.x - window.width() / 2.0,
        -(cursor_position.y - window.height() / 2.0),
    );
    (mousepos.x, mousepos.y) = (world_pos.x, world_pos.y);
}

// fn handle_input(
//     mut commands: Commands,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mouse_button_input: Res<ButtonInput<MouseButton>>,
//     mut query: Query<(&mut Transform), With<Player>>,
//     mousepos: ResMut<MousePosWorld>,
// ) {
//     let mut direction: f32 = 0.0;
//     if keyboard_input.pressed(KeyCode::KeyA) {
//         direction -= 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::KeyD) {
//         direction += 1.0;
//     }

// }
