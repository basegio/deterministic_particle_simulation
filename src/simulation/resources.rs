use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationSettings {
    pub size: f32,
    pub max_particle_radius: f32,
    pub gravity: GravityMode,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            size: 1024.0,
            max_particle_radius: 2.0,
            // gravity: GravityMode::Constant(Vec2::new(0.0, -9.8)),
            gravity: GravityMode::Point(Vec2::ZERO, 24000.0),
            // gravity: GravityMode::None,
        }
    }
}

#[derive(Resource)]
pub enum GravityMode {
    Constant(Vec2),
    Point(Vec2, f32),
    None,
}
