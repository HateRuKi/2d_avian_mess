use crate::game::game::{
    GRIDSIZE, GameLayer, GridCoord, GridMap, GridSelectorCoord, GridSelectorPos, MAXPLACEDISTANCE,
    Player,
};
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
pub struct BlockPlugin;
impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (block_system,));
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

    if !player_overlap && in_reach && (mousebutton.just_pressed(MouseButton::Right)) {
        let gridcoord = GridCoord {
            x: gridselectorcoord.x,
            y: gridselectorcoord.y,
        };
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
                    [GameLayer::Default, GameLayer::Player, GameLayer::Enemy],
                ),
            ))
            .id();

        if let Some(old_entities) = gridmap.0.insert(gridcoord, vec![entity]) {
            for old_entity in old_entities {
                cmd.entity(old_entity).despawn();
            }
        }

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
    // println!("printing GridMap");

    // println!("{:?}", gridmap);
}

