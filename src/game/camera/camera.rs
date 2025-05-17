use avian2d::prelude::*;
use bevy::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}

fn camera_setup(mut cmd: Commands) {
    println!("Hello From camera.rs!");
    cmd.spawn(Camera2d)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0));
}
