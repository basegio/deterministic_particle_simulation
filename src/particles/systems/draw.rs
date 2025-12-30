use bevy::prelude::*;

use crate::particles::components::Particle;

pub fn draw_particles(mut gizmos: Gizmos, query: Query<&Transform, With<Particle>>) {
    for transform in &query {
        // TODO: estudar como puxar Particle junto com Transform
        let particle_radius = 4.0;
        gizmos.circle_2d(transform.translation.xy(), particle_radius, Color::WHITE);
    }
}
