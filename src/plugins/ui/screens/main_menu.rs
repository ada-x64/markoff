use bevy::prelude::*;
use bevy_hui::prelude::*;

use crate::ui::screens::{CurrentScreen, ScreenMarker};

pub struct MainMenuScreenPlugin;
impl Plugin for MainMenuScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register)
            .add_systems(OnEnter(CurrentScreen::MainMenu), render);
    }
}

fn render(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        ScreenMarker,
        HtmlNode(server.load("hui/screens/main_menu.xml")),
    ));
}

fn register(mut html_funcs: HtmlFunctions) {
    // html_comps.register("hui/screens/main_menu/...")
    html_funcs.register(
        "goto_game_settings",
        |In(_entity), mut state: ResMut<NextState<CurrentScreen>>| {
            info!("goto_game_settings");
            state.set(CurrentScreen::GameSettings);
        },
    );
    html_funcs.register(
        "goto_sandbox",
        |In(_entity), mut state: ResMut<NextState<CurrentScreen>>| {
            state.set(CurrentScreen::Sandbox);
        },
    );
}
