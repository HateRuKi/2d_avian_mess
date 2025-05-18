use crate::game::game::{GRIDSIZE, GridSelectorCoord, GridSelectorPos, MousePosWorld};
use avian2d::{math::*, prelude::*};
use bevy::{gizmos::grid, prelude::*};
pub struct GridSystemPlugin;
impl Plugin for GridSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawngridselector)
            .add_systems(Update, (gridselect));
    }
}
#[derive(Component)]
struct GridSelector;

fn spawngridselector(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Hello From grid_system.rs!");
    let line_thickness = 0.1;
    let calculated_thickness = GRIDSIZE * line_thickness;
    let calculated_move = (GRIDSIZE * 0.5);
    cmd.spawn((
        GridSelector,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh2d(meshes.add(Rectangle::new(GRIDSIZE, GRIDSIZE))),
        MeshMaterial2d(materials.add(Color::srgba_u8(255, 255, 255, 100))),
    ))
    .with_children(|parent| {
        parent.spawn((
            MeshMaterial2d(materials.add(Color::srgba_u8(255, 255, 255, 255))),
            Transform::from_xyz(calculated_move, 0.0, 0.0),
            Mesh2d(meshes.add(Rectangle::new(calculated_thickness, GRIDSIZE))),
        ));
        parent.spawn((
            MeshMaterial2d(materials.add(Color::srgba_u8(255, 255, 255, 255))),
            Transform::from_xyz(-calculated_move, 0.0, 0.0),
            Mesh2d(meshes.add(Rectangle::new(calculated_thickness, GRIDSIZE))),
        ));
        parent.spawn((
            MeshMaterial2d(materials.add(Color::srgba_u8(255, 255, 255, 255))),
            Transform::from_xyz(0.0, calculated_move, 0.0),
            Mesh2d(meshes.add(Rectangle::new(GRIDSIZE, calculated_thickness))),
        ));
        parent.spawn((
            MeshMaterial2d(materials.add(Color::srgba_u8(255, 255, 255, 255))),
            Transform::from_xyz(0.0, -calculated_move, 0.0),
            Mesh2d(meshes.add(Rectangle::new(GRIDSIZE, calculated_thickness))),
        ));
    });

    for i in -10..11 {
        for j in -10..11 {
            cmd.spawn((
                Mesh2d(meshes.add(Circle::new(5.0))),
                MeshMaterial2d(materials.add(Color::srgba_u8(255, 255, 255, 255))),
                Transform::from_xyz(i as f32 * GRIDSIZE, j as f32 * GRIDSIZE, 0.0),
            ));
        }
    }
}

fn gridselect(
    mouseposworld: Res<MousePosWorld>,
    mut query_gridselector: Query<(&mut Transform), With<GridSelector>>,
    mut gridselectorpos: ResMut<GridSelectorPos>,
    mut gridselectorcoord: ResMut<GridSelectorCoord>,
) {
    // println!(
    //     "Grid Selector >> {} , {}",
    //     gridselectorpos.x, gridselectorpos.y
    // );
    let index_x = (mouseposworld.x / GRIDSIZE).floor();
    let index_y = (mouseposworld.y / GRIDSIZE).floor();
    for (mut gridselectortransform) in query_gridselector.iter_mut() {
        gridselectortransform.translation.x = GRIDSIZE * (index_x + 0.5);
        gridselectortransform.translation.y = GRIDSIZE * (index_y + 0.5);

        (gridselectorpos.x, gridselectorpos.y) = (
            gridselectortransform.translation.x,
            gridselectortransform.translation.y,
        );
        (gridselectorcoord.x, gridselectorcoord.y) = (index_x as i32, index_y as i32);
    }
}
