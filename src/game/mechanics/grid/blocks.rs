use crate::game::game::{
    BlockBreakEvent, BlockPlaceEvent, GRIDSIZE, GameLayer, GridCoord, GridMap, GridSelector,
    GridSelectorCoord, GridSelectorPos, MAXPLACEDISTANCE, Player,
};
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
pub struct BlockPlugin;
impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (block_system, block_break_handler, block_place_handler),
        ).add_event::<BlockPlaceEvent>().add_event::<BlockBreakEvent>();
    }
}

fn block_break_handler(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mousebutton: Res<ButtonInput<MouseButton>>,
    mut gridmap: ResMut<GridMap>,
    mut gridselectorcoord: Res<GridSelectorCoord>,
    mut gridselectorpos: Res<GridSelectorPos>,
    player_query: Query<&Player, With<Player>>,
    mut gridselector_query: Query<Entity, With<GridSelector>>,
    mut block_break_events: EventReader<BlockBreakEvent>,
) {
    for event in block_break_events.read() {
        let gridcoord = GridCoord {
            x: gridselectorcoord.x,
            y: gridselectorcoord.y,
        };
        if let Some(entities) = gridmap.0.remove(&gridcoord) {
            for entity in entities {
                cmd.entity(entity).despawn();
            }
        }
    }
    
}
fn block_place_handler(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mousebutton: Res<ButtonInput<MouseButton>>,
    mut gridmap: ResMut<GridMap>,
    mut gridselectorcoord: Res<GridSelectorCoord>,
    mut gridselectorpos: Res<GridSelectorPos>,
    player_query: Query<&Player, With<Player>>,
    mut gridselector_query: Query<Entity, With<GridSelector>>,
    mut block_place_events: EventReader<BlockPlaceEvent>,
) {
    let gridcoord = GridCoord {
        x: gridselectorcoord.x,
        y: gridselectorcoord.y,
    };
    for event in block_place_events.read() {
        let entity = cmd
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(GRIDSIZE, GRIDSIZE))),
                MeshMaterial2d(materials.add(Color::srgba_u8(150, 150, 150, 255))),
                Transform::from_xyz(
                    (gridcoord.x as f32 * GRIDSIZE) + GRIDSIZE * 0.5,
                    (gridcoord.y as f32 * GRIDSIZE) + GRIDSIZE * 0.5,
                    0.0,
                ),
                gridcoord,
                RigidBody::Static,
                Collider::rectangle(GRIDSIZE, GRIDSIZE),
                CollisionLayers::new(
                    GameLayer::Block,
                    [
                        GameLayer::Default,
                        GameLayer::Player,
                        GameLayer::Enemy,
                        GameLayer::Projectile,
                    ],
                ),
            ))
            .id();

        if let Some(old_entities) = gridmap.0.insert(gridcoord, vec![entity]) {
            for old_entity in old_entities {
                cmd.entity(old_entity).despawn();
            }
        }
    }
}

fn block_system(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mousebutton: Res<ButtonInput<MouseButton>>,
    mut gridmap: ResMut<GridMap>,
    mut gridselectorcoord: Res<GridSelectorCoord>,
    mut gridselectorpos: Res<GridSelectorPos>,
    player_query: Query<&Player, With<Player>>,
    mut gridselector_query: Query<Entity, With<GridSelector>>,
    mut block_place_events: EventWriter<BlockPlaceEvent>,
    mut block_break_events: EventWriter<BlockBreakEvent>,
) {
    let distance: f32 = {
        let mut dist: f32 = GRIDSIZE;
        for player_component in player_query.iter() {
            dist = (player_component.position.truncate()
                - Vec2::new(gridselectorpos.x, gridselectorpos.y))
            .length();
        }
        dist
    };
    // println!("{:?}", distance);
    let player_overlap = distance < GRIDSIZE;
    let in_reach = distance < MAXPLACEDISTANCE;

    for entity in gridselector_query {
        if !player_overlap && in_reach {
            cmd.entity(entity).insert(MeshMaterial2d(
                materials.add(Color::srgba_u8(175, 250, 175, 100)),
            ));
        } else {
            cmd.entity(entity).insert(MeshMaterial2d(
                materials.add(Color::srgba_u8(250, 175, 175, 100)),
            ));
        }
    }

    if !player_overlap && in_reach && (mousebutton.just_pressed(MouseButton::Right)) {
        block_place_events.write(BlockPlaceEvent);
        // moved to event
        // let gridcoord = GridCoord {
        //     x: gridselectorcoord.x,
        //     y: gridselectorcoord.y,
        // };
        // let entity = cmd
        //     .spawn((
        //         Mesh2d(meshes.add(Rectangle::new(GRIDSIZE, GRIDSIZE))),
        //         MeshMaterial2d(materials.add(Color::srgba_u8(150, 150, 150, 255))),
        //         Transform::from_xyz(
        //             (gridcoord.x as f32 * GRIDSIZE) + GRIDSIZE * 0.5,
        //             (gridcoord.y as f32 * GRIDSIZE) + GRIDSIZE * 0.5,
        //             0.0,
        //         ),
        //         gridcoord,
        //         RigidBody::Static,
        //         Collider::rectangle(GRIDSIZE, GRIDSIZE),
        //         CollisionLayers::new(
        //             GameLayer::Block,
        //             [
        //                 GameLayer::Default,
        //                 GameLayer::Player,
        //                 GameLayer::Enemy,
        //                 GameLayer::Projectile,
        //             ],
        //         ),
        //     ))
        //     .id();

        // if let Some(old_entities) = gridmap.0.insert(gridcoord, vec![entity]) {
        //     for old_entity in old_entities {
        //         cmd.entity(old_entity).despawn();
        //     }
        // }

        // // NOTE
        // // for later
        // // multiple entities in one tile
        // let cell = gridmap.0.entry(gridcoord).or_default();
        // if !cell.contains(&entity) {
        //     cell.push(entity);
        // }

        //removed lol mf caused duplicates
        // gridmap.0.entry(gridcoord).or_default().push(entity);
    }
    if in_reach && mousebutton.just_pressed(MouseButton::Left) {

                block_break_events.write(BlockBreakEvent);

        // let gridcoord = GridCoord {
        //     x: gridselectorcoord.x,
        //     y: gridselectorcoord.y,
        // };
        // if let Some(entities) = gridmap.0.remove(&gridcoord) {
        //     for entity in entities {
        //         cmd.entity(entity).despawn();
        //     }
        // }
    }
    // println!("printing GridMap");

    // println!("{:?}", gridmap);
}
