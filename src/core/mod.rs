use bevy::prelude::*;

mod window;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, window::log_fps);
    }
}
