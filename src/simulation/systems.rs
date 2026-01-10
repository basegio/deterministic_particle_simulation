use super::resources::SimulationSettings;

use bevy::prelude::*;

pub fn draw_constraints(mut gizmos: Gizmos, settings: Res<SimulationSettings>) {
    let simulation_size = settings.size as f32;
    gizmos.rect_2d(
        Vec2::ZERO,
        Vec2::new(simulation_size, simulation_size) + Vec2::new(1.0, 1.0),
        Color::linear_rgb(1., 0., 0.),
    );
}
