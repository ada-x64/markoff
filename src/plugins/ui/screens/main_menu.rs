use bevy::prelude::*;
use bevy_flair::style::components::NodeStyleSheet;
use bevy_hui::prelude::*;

use crate::ui::{
    RootNode,
    screens::{CurrentScreen, ScreenRoot},
};

pub struct MainMenuScreenPlugin;
impl Plugin for MainMenuScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register)
            .add_systems(OnEnter(CurrentScreen::MainMenu), render);
    }
}

fn render(mut commands: Commands, server: Res<AssetServer>, root: Single<Entity, With<RootNode>>) {
    info!("main menu");
    commands.entity(*root).with_child((
        ScreenRoot,
        HtmlNode(server.load("hui/screens/main_menu.xml")),
        NodeStyleSheet::StyleSheet(server.load("hui/styles/main_menu.css")),
    ));
}

fn register(
    mut html_funcs: HtmlFunctions,
    mut html_components: HtmlComponents,
    server: Res<AssetServer>,
) {
    html_components.register("main_menu", server.load("hui/screens/main_menu.xml"));
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
