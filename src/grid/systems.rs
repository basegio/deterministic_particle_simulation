use bevy::prelude::*;

use crate::grid::resource::CollisionGrid;
use crate::particles::components::Particle;

pub fn update_grid(mut grid: ResMut<CollisionGrid>, query: Query<(Entity, &Particle)>) {
    grid.clear();
    for (entity, particle) in &query {
        if let Some(idx) = grid.get_cell_index_from_particle_pos(particle.position) {
            grid.cells[idx].push(entity);
        }
    }
}
