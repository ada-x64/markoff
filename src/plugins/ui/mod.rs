use bevy::prelude::*;
pub mod data;
pub mod screens;
pub mod widgets;

use bevy_flair::{FlairPlugin, style::components::NodeStyleSheet};
use bevy_hui::{
    HuiPlugin,
    prelude::{CompileContextEvent, HuiAutoLoadPlugin, Tags},
};
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
            .add_observer(set_styles)
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

// goal: Whenever the HTML is recompiled or populated,
// we need to go through and parse tags so that we can integrate with
// bevy_flair. tag:id and tag:class should correlate to the Name and ClassList
// components, respectively.
// issue: CompileContextEvent is triggered locally, doesn't bubble up
fn set_styles(trigger: Trigger<CompileContextEvent>, tags: Query<&Tags>) {}
