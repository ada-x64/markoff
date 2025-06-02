use bevy::prelude::*;

pub mod main_menu;

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct ScreenMarker;

#[derive(States, Copy, Clone, Default, Hash, PartialEq, Eq, Debug)]
pub enum Screen {
    #[default]
    MainMenu,
    GameSettings,
    MainLoop,
    Results,
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
                .add_systems(
                    Update,
                    (main_menu::update).run_if(in_state(Screen::MainMenu)),
                )
                .add_systems(OnEnter(Screen::MainMenu), main_menu::init)
        };
    }
}
