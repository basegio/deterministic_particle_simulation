use bevy::prelude::*;

use crate::particles::components::Particle;
use crate::simulation::resources::SimulationSettings;

pub fn draw_particles(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<Particle>>,
    settings: Res<SimulationSettings>,
) {
    for transform in &query {
        gizmos.circle_2d(
            transform.translation.xy() * settings.zoom + Vec2::new(settings.zoom / 2.0, 0.0),
            settings.zoom / 2.0,
            Color::WHITE,
        );
    }
}
