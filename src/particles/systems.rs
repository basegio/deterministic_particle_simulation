use crate::particles::components::Particle;
use crate::simulation::resources::SimulationSettings;
use bevy::prelude::*;

pub fn spawn_particles(mut commands: Commands, settings: Res<SimulationSettings>) {
    for i in 0..32 {
        // if i % 2 != 0 {
        //     continue;
        // }

        let x: f32 = i as f32 - settings.size / 2.0;
        commands.spawn((Particle::default(), Transform::from_xyz(x, 0.0, 0.0)));
    }
}

pub fn apply_physics(
    time: Res<Time<Fixed>>,
    settings: Res<SimulationSettings>,
    mut query: Query<(&mut Transform, &mut Particle)>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut particle) in &mut query {
        particle.velocity += settings.gravity * dt;
        transform.translation += particle.velocity.extend(0.0) * dt;
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
