use crate::particles::components::Particle;
use crate::simulation::resources::SimulationSettings;

pub enum BoundaryMode {
    None,
    Square,
    Circle { radius: f32 },
}

impl BoundaryMode {
    pub fn resolve(&self, settings: &SimulationSettings, particles: &mut Vec<Particle>) {
        match self {
            BoundaryMode::None => {}
            BoundaryMode::Square => solve_square_boundary(settings, particles),
            BoundaryMode::Circle { radius } => solve_circle_boundary(settings, particles, *radius),
        }
    }
}

#[inline(always)]
fn solve_square_boundary(settings: &SimulationSettings, particles: &mut Vec<Particle>) {
    let border_distance = settings.size as f32 / 2.0;

    for particle in particles {
        if particle.position.y < -border_distance + particle.radius {
            particle.position.y = -border_distance + particle.radius
        } else if particle.position.y > border_distance - particle.radius {
            particle.position.y = border_distance - particle.radius
        }

        if particle.position.x < -border_distance + particle.radius {
            particle.position.x = -border_distance + particle.radius
        } else if particle.position.x > border_distance - particle.radius {
            particle.position.x = border_distance - particle.radius
        }
    }
}

#[inline(always)]
fn solve_circle_boundary(
    settings: &SimulationSettings,
    particles: &mut Vec<Particle>,
    radius: f32,
) {
    for particle in particles {
        let max_dist = radius - particle.radius;
        let dist_sq = particle.position.length_squared();

        if dist_sq > max_dist * max_dist {
            let dist = dist_sq.sqrt();
            let n = particle.position / dist;
            let delta = dist - max_dist;

            let compensation = n * delta;
            particle.position -= compensation;
            particle.position_old += n * delta * settings.restitution_amortization;
        }
    }
}
