use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationSettings {
    pub size: usize,
    pub min_particle_radius: f32,
    pub max_particle_radius: f32,
    pub gravity: GravityMode,
    pub substeps: usize,
    pub damping: f32,
    pub restitution_amortization: f32,
    pub relaxation_factor: f32,
    pub softening: f32,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            size: 1024,
            min_particle_radius: 2.0,
            max_particle_radius: 8.0,
            // gravity: GravityMode::Constant(Vec2::new(0.0, -9.8)),
            gravity: GravityMode::Point(Vec2::ZERO, 72000.0),
            // gravity: GravityMode::None,
            substeps: 8,
            damping: 0.99,
            restitution_amortization: 0.05,
            relaxation_factor: 0.2,
            softening: 40000.0,
        }
    }
}

pub enum GravityMode {
    Constant(Vec2),
    Point(Vec2, f32),
    None,
}
