use bevy::prelude::*;

use crate::grid::resource::CollisionGrid;

pub mod resource;
pub mod systems;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        // TODO: estudar como puxar Particle junto com Transform
        // deve ser max_particle_radius
        // worldSize deve vir junto do SimulationSettings
        let particle_radius = 4.0;
        let grid = CollisionGrid::new(512.0, particle_radius * 2.0);
        app.insert_resource(grid)
            .add_systems(FixedUpdate, systems::update_grid);
    }
}
