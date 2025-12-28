use bevy::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            velocity: Vec2::ZERO,
        }
    }
}
