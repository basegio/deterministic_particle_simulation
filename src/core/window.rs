use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

pub fn log_fps(diagnostics: Res<DiagnosticsStore>, mut window: Single<&mut Window>) {
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        window.title = format!("DNS particle simulation | {:.1} fps", value);
    }
}
