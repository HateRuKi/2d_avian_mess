use crate::game::game::{GameLayer, GridCoord, GridMap, GridSelectorCoord};
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
) {
    let gridsize = 50.0;
    if mousebutton.just_pressed(MouseButton::Right) {
        let gridcoord = GridCoord {
            x: gridselectorcoord.x,
            y: gridselectorcoord.y,
        };
        let entity = cmd
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(gridsize, gridsize))),
                MeshMaterial2d(materials.add(Color::srgba_u8(150, 150, 150, 255))),
                Transform::from_xyz(
                    (gridcoord.x as f32 * gridsize) + gridsize * 0.5,
                    (gridcoord.y as f32 * gridsize) + gridsize * 0.5,
                    0.0,
                ),
                gridcoord,
                RigidBody::Static,
                Collider::rectangle(gridsize, gridsize),
                CollisionLayers::new(
                    GameLayer::Block,
                    [GameLayer::Default, GameLayer::Player, GameLayer::Enemy],
                ),
            ))
            .id();
        gridmap.0.entry(gridcoord).or_default().push(entity);
    }
    if mousebutton.just_pressed(MouseButton::Left) {
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
