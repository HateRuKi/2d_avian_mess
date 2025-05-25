use std::{collections::HashMap, fmt};

use avian2d::prelude::*;
use bevy::prelude::*;

use super::{
    camera::camera,
    input::input,
    level::level,
    mechanics::{mechanics, projectile},
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
            projectile::ProjectilePlugin,
        ))
        .insert_resource(MousePosWorld { x: 0.0, y: 0.0 })
        .insert_resource(MousePosWindow { x: 0.0, y: 0.0 })
        .insert_resource(GridSelectorPos { x: 0.0, y: 0.0 })
        .insert_resource(GridSelectorCoord { x: 0, y: 0 })
        .insert_resource(GridMap::default());
    }
}

pub const GRIDSIZE: f32 = 50.0;
pub const GRAVITY: f32 = 10.0;
pub const MAXPLACEDISTANCE: f32 = 300.0;
pub const PLAYERSIZE: f32 = 50.0;
#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
    Ground,
    Object,
    Block,
    Projectile,
}
#[derive(Component)]
pub struct JumpSensor;
#[derive(Component)]
pub struct Player {
    pub acceleration: f32,
    pub damping: f32,
    pub max_speed: f32,
    pub position: Vec3,
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
#[derive(Component)]
pub struct GridSelector;
#[derive(Resource, Default)]
pub struct GridMap(pub HashMap<GridCoord, Vec<Entity>>);
impl GridMap {
    pub fn total_entity_count(&self) -> usize {
        self.0.values().map(Vec::len).sum()
    }
}
impl fmt::Debug for GridMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GridMap (total entities: {})", self.total_entity_count())?;
        for (coord, entities) in &self.0 {
            writeln!(
                f,
                "  {:?}: {} entities -> {:?}",
                coord,
                entities.len(),
                entities
            )?;
        }
        Ok(())
    }
}
#[derive(Copy, Clone)]
pub enum TrajectoryType {
    Projectile,
    Straight,
}
#[derive(Copy, Clone)]
pub enum SpeedType {
    Velocity,
    Accerleration,
}
#[derive(Component)]
pub struct ProjectileAttributes {
    // pub entity: Entity,
    pub trajectory_type: TrajectoryType,
    pub speed_type: SpeedType,
    pub speed: f32,
    pub direction: Vec2,
}
#[derive(Event, Copy, Clone)]
pub struct ProjectileCreationEvent {
    pub entity: Entity,
    pub trajectory_type: TrajectoryType,
    pub speed_type: SpeedType,
    pub speed: f32,
    pub direction: Vec2,
    pub origin: Vec2,
}

#[derive(Event, Copy, Clone)]
pub struct BlockPlaceEvent ;
#[derive(Event, Copy, Clone)]
pub struct BlockBreakEvent ;