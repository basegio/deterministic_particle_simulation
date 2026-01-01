use std::time::Instant;

use bevy::prelude::*;

use crate::core::diagnostic::resource::GridUpdateDiagnostic;
use crate::grid::resource::CollisionGrid;
use crate::particles::components::Particle;

pub fn update_grid(
    mut grid: ResMut<CollisionGrid>,
    query: Query<(Entity, &Particle)>,
    mut diag: ResMut<GridUpdateDiagnostic>,
) {
    let start = Instant::now();

    grid.clear();
    for (entity, particle) in &query {
        if let Some(idx) = grid.get_cell_index_from_particle_pos(particle.position) {
            grid.cells[idx].push(entity);
        }
    }

    let duration = start.elapsed();
    diag.times.push(duration);

    if diag.times.len() == diag.max_samples {
        diag.print_result();
    }
}
