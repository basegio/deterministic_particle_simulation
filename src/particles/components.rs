use bevy::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub position: Vec2,
    pub position_old: Vec2,
    pub radius: f32,
}

impl Particle {
    pub fn from_position(position: Vec2) -> Self {
        Particle {
            position: position,
            position_old: position,
            radius: 4.0,
        }
    }
}
