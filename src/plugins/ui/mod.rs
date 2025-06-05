use bevy::prelude::*;
pub mod screens;
// pub mod theme;
// pub mod widgets;

use bevy_hui::{
    HuiPlugin,
    prelude::{HtmlComponents, HuiAutoLoadPlugin},
};
use screens::*;
// use theme::*;
// use widgets::*;

#[derive(Resource, Debug, Clone)]
pub struct UiAssets {
    pub bg1: Handle<Image>,
}
impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let bg1 = world.load_asset("textures/bg1.png");
        Self { bg1 }
    }
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.add_plugins((ScreensPlugin, HuiPlugin))
                .init_resource::<UiAssets>()
                .add_systems(
                    PreStartup,
                    |mut pick_settings: ResMut<UiPickingSettings>| {
                        *pick_settings = UiPickingSettings {
                            require_markers: true,
                        }
                    },
                )
                .add_systems(Startup, (spawn_camera, register_widgets))
        };
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, UiPickingCamera));
}

fn register_widgets(mut html_comps: HtmlComponents, server: Res<AssetServer>) {
    let mut register = |name: &str| {
        html_comps.register(name, server.load(format!("hui/components/{name}.xml")));
    };
    register("menu_button");
    register("grid_layout");
    register("row");
    register("column");
}
