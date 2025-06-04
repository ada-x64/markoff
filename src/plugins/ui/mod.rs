use bevy::prelude::*;
pub mod screens;
pub mod theme;
pub mod widgets;

use screens::*;
use theme::*;
use widgets::*;

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
            app.add_plugins((WidgetsPlugin, ScreensPlugin, ThemePlugin))
                .init_resource::<UiAssets>()
                .add_systems(
                    PreStartup,
                    |mut pick_settings: ResMut<UiPickingSettings>| {
                        *pick_settings = UiPickingSettings {
                            require_markers: true,
                        }
                    },
                )
                .add_systems(Startup, spawn_camera)
        };
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, UiPickingCamera));
    info!("Spawning camera!");
}
