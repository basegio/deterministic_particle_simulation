pub mod components;
pub mod systems;

use bevy::prelude::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_particles)
            .add_systems(
                FixedUpdate,
                (
                    systems::apply_physics,
                    systems::solve_collisions,
                    systems::solve_enviroment_constraints_limits,
                ),
            )
            .add_systems(Update, systems::draw_particles);
    }
}
