use bevy::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

use crate::ui::screens::{main_menu::MainMenuScreenPlugin, sandbox::SandboxScreenPlugin};

// pub mod main_loop;
pub mod main_menu;
pub mod sandbox;

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct ScreenMarker;

#[derive(States, Copy, Clone, Default, Hash, PartialEq, Eq, Debug, EnumIter)]
pub enum CurrentScreen {
    #[default]
    MainMenu,
    GameSettings,
    MainLoop,
    Results,
    Sandbox,
}

#[macro_export]
macro_rules! load_template {
    ($template:expr) => {
        |mut cmd: Commands, server: Res<AssetServer>| {
            cmd.spawn(HtmlNode(server.load($template)));
        }
    };
}

pub struct ScreensPlugin;
impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.init_state::<CurrentScreen>()
                .add_plugins(MainMenuScreenPlugin)
                .add_plugins(SandboxScreenPlugin)
        };
        for screen in CurrentScreen::iter() {
            app.add_systems(OnExit(screen), cleanup_screen);
        }
    }
}

fn cleanup_screen(screens: Query<Entity, With<ScreenMarker>>, mut commands: Commands) {
    for screen in screens {
        commands.get_entity(screen).unwrap().despawn();
    }
}
