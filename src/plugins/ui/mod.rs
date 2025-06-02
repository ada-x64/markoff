use bevy::prelude::*;
pub mod screens;
pub mod widgets;

use screens::*;
use widgets::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.add_plugins((WidgetsPlugin, ScreensPlugin))
                .add_systems(Startup, spawn_camera)
        };
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Spawning camera!");
}
