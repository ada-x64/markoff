use bevy::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

pub mod main_loop;
pub mod main_menu;
pub mod sandbox;

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct ScreenMarker;

#[derive(States, Copy, Clone, Default, Hash, PartialEq, Eq, Debug, EnumIter)]
pub enum Screen {
    MainMenu,
    GameSettings,
    #[default]
    MainLoop,
    Results,
    Sandbox,
}

#[macro_export]
macro_rules! next_screen {
    ($state:expr) => {
        |mut next_state: ResMut<NextState<Screen>>| {
            next_state.set($state);
        }
    };
}

pub struct ScreensPlugin;
impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.init_state::<Screen>()
                .add_systems(OnEnter(Screen::MainMenu), main_menu::init)
                .add_systems(OnEnter(Screen::Sandbox), sandbox::init)
                .add_systems(OnEnter(Screen::MainLoop), main_loop::init)
        };
        for screen in Screen::iter() {
            app.add_systems(OnExit(screen), cleanup_screen);
        }
    }
}

fn cleanup_screen(screens: Query<Entity, With<ScreenMarker>>, mut commands: Commands) {
    for screen in screens {
        commands.get_entity(screen).unwrap().despawn();
    }
}
