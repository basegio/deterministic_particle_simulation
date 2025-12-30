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
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(CorePlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(ParticlePlugin)
        .run();
}
