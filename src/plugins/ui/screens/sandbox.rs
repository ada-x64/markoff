use bevy::prelude::*;
use bevy_hui::prelude::*;

use crate::{
    sim::{SimSettings, SimState},
    ui::screens::{CurrentScreen, ScreenMarker},
};

pub fn render(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        ScreenMarker,
        HtmlNode(server.load("hui/screens/sandbox.xml")),
    ));
}

pub fn register(mut cmds: Commands, mut html_comps: HtmlComponents, mut html_funcs: HtmlFunctions) {
    html_funcs.register(
        "load_sim",
        |In(entity),
         mut sim_state: ResMut<NextState<SimState>>,
         mut settings: ResMut<SimSettings>| {
            settings.parent_node = Some(entity);
            sim_state.set(SimState::Init);
        },
    )
}
