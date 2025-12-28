use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationSettings {
    pub size: f32,
    pub zoom: f32,
    pub gravity: Vec2,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            size: 32.0,
            zoom: 8.0,
            gravity: Vec2::new(0.0, 0.0),
        }
    }
}
