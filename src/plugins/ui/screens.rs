use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::ui::{
    data::{CurrentScreen, ScreenRoot},
    screens::{
        init::InitScreenPlugin, main_menu::MainMenuScreenPlugin, sandbox::SandboxScreenPlugin,
    },
};

pub mod init;
pub mod main_menu;
pub mod sandbox;

pub struct ScreensPlugin;
impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.init_state::<CurrentScreen>()
                .add_plugins(InitScreenPlugin)
                .add_plugins(MainMenuScreenPlugin)
                .add_plugins(SandboxScreenPlugin)
        };
        for screen in CurrentScreen::iter() {
            app.add_systems(OnExit(screen), cleanup_screen);
        }
    }
}

fn cleanup_screen(screens: Query<Entity, With<ScreenRoot>>, mut commands: Commands) {
    for screen in screens {
        commands.get_entity(screen).unwrap().despawn();
    }
}
