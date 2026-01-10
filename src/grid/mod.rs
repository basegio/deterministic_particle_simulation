use bevy::prelude::*;

use crate::grid::resource::CollisionGrid;

pub mod resource;
pub mod systems;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollisionGrid>();
    }
}
