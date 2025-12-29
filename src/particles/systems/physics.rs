use crate::particles::components::Particle;
use crate::simulation::resources::SimulationSettings;
use bevy::prelude::*;

pub fn solve_collisions(mut query: Query<(Entity, &mut Particle)>) {
    let mut combinations = query.iter_combinations_mut();

    while let Some([(_, mut p_a), (_, mut p_b)]) = combinations.fetch_next() {
        let collision_axis = p_a.position - p_b.position;
        let dist_sq = collision_axis.length_squared();

        let particle_radius = 1.0 / 2.0;
        let min_dist = particle_radius + particle_radius;

        if dist_sq < min_dist * min_dist {
            let dist = dist_sq.sqrt();
            let n = collision_axis / dist;
            let delta = min_dist - dist;
            let compensation = n * delta * 0.5;
            p_a.position += compensation;
            p_b.position -= compensation;
        }
    }
}

pub fn solve_enviroment_constraints_limits(
    settings: Res<SimulationSettings>,
    mut query: Query<(&mut Transform, &mut Particle)>,
) {
    let particle_radius = 1.0 / 2.0;
    let border_distance = settings.size / 2.0;

    for (mut transform, mut particle) in &mut query {
        if particle.position.y < -border_distance + particle_radius {
            particle.position.y = -border_distance + particle_radius
        } else if particle.position.y > border_distance - particle_radius {
            particle.position.y = border_distance - particle_radius
        }

        if particle.position.x < -border_distance + particle_radius {
            particle.position.x = -border_distance + particle_radius
        } else if particle.position.x > border_distance - particle_radius {
            particle.position.x = border_distance - particle_radius
        }

        transform.translation = particle.position.extend(0.0);
    }
}
