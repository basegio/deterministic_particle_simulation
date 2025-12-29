use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationSettings {
    pub size: f32,
    pub zoom: f32,
    pub gravity: GravityMode,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            size: 32.0,
            zoom: 8.0,
            // gravity: GravityMode::Constant(Vec2::new(0.0, -9.8)),
            gravity: GravityMode::Point(Vec2::ZERO, 10.0),
        }
    }
}

#[derive(Resource)]
pub enum GravityMode {
    Constant(Vec2),
    Point(Vec2, f32),
    None,
}
