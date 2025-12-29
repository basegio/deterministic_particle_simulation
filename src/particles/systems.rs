use crate::particles::components::Particle;
use crate::simulation::resources::{GravityMode, SimulationSettings};
use bevy::math::VectorSpace;
use bevy::prelude::*;

pub fn spawn_particles(mut commands: Commands, settings: Res<SimulationSettings>) {
    for i in 0..32 {
        for j in 0..4 {
            if i % 2 != 0 || j % 2 != 0 {
                continue;
            }

            let x: f32 = i as f32 - settings.size / 2.0;
            let y: f32 = j as f32;
            let position: Vec2 = Vec2 { x: x, y: y };
            commands.spawn((
                Particle::from_position(position),
                Transform::from_xyz(position.x, position.y, 0.0),
            ));
        }
    }
}

pub fn apply_physics(
    time: Res<Time<Fixed>>,
    settings: Res<SimulationSettings>,
    mut query: Query<(&mut Transform, &mut Particle)>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut particle) in &mut query {
        let acceleration = match settings.gravity {
            GravityMode::Constant(g) => g,
            GravityMode::Point(center, strenght) => {
                let dir = center - particle.position;
                let dist_sq = dir.length_squared().max(0.1);
                (dir.normalize_or_zero() * strenght) / dist_sq
            }
            GravityMode::None => Vec2::ZERO,
        };

        let velocity = particle.position - particle.position_old;
        particle.position_old = particle.position;
        particle.position = particle.position + velocity + acceleration * (dt * dt);
        transform.translation = particle.position.extend(0.0);
    }
}

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

pub fn draw_particles(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<Particle>>,
    settings: Res<SimulationSettings>,
) {
    for transform in &query {
        gizmos.circle_2d(
            transform.translation.xy() * settings.zoom + Vec2::new(settings.zoom / 2.0, 0.0),
            settings.zoom / 2.0,
            Color::WHITE,
        );
    }
}
