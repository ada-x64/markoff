use bevy::prelude::*;
pub mod data;
pub mod screens;
pub mod widgets;

use bevy_flair::{FlairPlugin, style::components::NodeStyleSheet};
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
                FlairPlugin,
                ScreensPlugin,
                WidgetsPlugin,
            ))
            .init_resource::<TemplateHandles>()
            .add_systems(Startup, (spawn_camera, spawn_root_node))
        };
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, UiPickingCamera));
}

#[derive(Component, Default, Debug)]
pub struct RootNode;

fn spawn_root_node(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        RootNode,
        Node::default(),
        NodeStyleSheet::StyleSheet(assets.load("hui/styles/root.css")),
    ));
}
