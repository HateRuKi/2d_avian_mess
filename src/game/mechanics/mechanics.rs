use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use super::grid::grid_system::GridSystemPlugin;
use super::grid::blocks::BlockPlugin;
pub struct MechanicsPlugin;
impl Plugin for MechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GridSystemPlugin,BlockPlugin));
    }
}
