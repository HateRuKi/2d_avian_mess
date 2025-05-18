use std::collections::HashMap;

use avian2d::prelude::*;
use bevy::prelude::*;

use super::{
    camera::camera,
    input::input,
    level::level,
    mechanics::mechanics,
    player::{movement, player},
};
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            level::LevelPlugin,
            player::PlayerPlugin,
            camera::CameraPlugin,
            input::InputPlugin,
            mechanics::MechanicsPlugin,
            movement::PlayerMovementPlugin,
        ))
        .insert_resource(MousePosWorld { x: 0.0, y: 0.0 })
        .insert_resource(MousePosWindow { x: 0.0, y: 0.0 })
        .insert_resource(GridSelectorPos { x: 0.0, y: 0.0 })
        .insert_resource(GridSelectorCoord { x: 0, y: 0 })
        .insert_resource(GridMap::default());
    }
}

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
    Ground,
    Object,
    Block,
}

#[derive(Component)]
pub struct Player {
    pub acceleration: f32,
    pub damping: f32,
    pub max_speed: f32,
}
#[derive(Resource)]
pub struct MousePosWorld {
    pub x: f32,
    pub y: f32,
}
#[derive(Resource)]
pub struct MousePosWindow {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource)]
pub struct GridSelectorPos {
    pub x: f32,
    pub y: f32,
}
#[derive(Resource)]
pub struct GridSelectorCoord {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoord {
    pub x: i32,
    pub y: i32,
}
#[derive(Resource, Default)]
pub struct GridMap(pub HashMap<GridCoord, Vec<Entity>>);
