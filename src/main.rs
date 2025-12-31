mod core;
mod grid;
mod particles;
mod simulation;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use core::CorePlugin;
use grid::GridPlugin;
use particles::ParticlePlugin;
use simulation::SimulationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (720, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(CorePlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(ParticlePlugin)
        .run();
}
