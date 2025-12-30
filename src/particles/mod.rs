pub mod components;
pub mod systems;

use bevy::prelude::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn::spawn_particles)
            .add_systems(
                FixedUpdate,
                (
                    systems::movement::apply_physics,
                    systems::physics::solve_collisions,
                    systems::physics::solve_enviroment_constraints_limits,
                )
                    .chain()
                    .after(crate::grid::systems::update_grid),
            )
            .add_systems(Update, systems::draw::draw_particles);
    }
}
