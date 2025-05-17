use avian2d::{math::*, prelude::*};
use bevy::{
    prelude::*,
    render::{render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};

mod game;
fn main() {
    App::new()
        .add_plugins((
            game::game::GamePlugin,
            DefaultPlugins,
            PhysicsDebugPlugin::default(),
            PhysicsPlugins::default().with_length_unit(20.0),
        ))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .add_systems(Startup, (setup))
        .run();
}

fn setup(mut cmd: Commands) {}

//a game about placing your own platform to avoid and kill flying enemies
