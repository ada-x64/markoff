use bevy::prelude::*;
pub mod data;
pub mod screens;
pub mod widgets;

use bevy_hui::{HuiPlugin, prelude::HuiAutoLoadPlugin};
use screens::*;

use crate::ui::{
    data::TemplateHandles,
    widgets::{Slider, WidgetsPlugin},
};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.add_plugins((
                // "components" are auto-loaded and don't require custom logic
                // "widgets" require custom logic and must be initialized manually
                HuiPlugin,
                HuiAutoLoadPlugin::new(&["hui/components"]),
                ScreensPlugin,
                WidgetsPlugin,
            ))
            .init_resource::<TemplateHandles>()
            .add_systems(Startup, spawn_camera)
        };
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, UiPickingCamera));
}
