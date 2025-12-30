use bevy::prelude::*;

use crate::particles::components::Particle;

pub fn draw_particles(mut gizmos: Gizmos, query: Query<(&Particle, &Transform)>) {
    for (particle, transform) in &query {
        gizmos.circle_2d(transform.translation.xy(), particle.radius, Color::WHITE);
    }
}
