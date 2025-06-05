use bevy::prelude::*;
use bevy_hui::prelude::*;

use crate::ui::screens::{CurrentScreen, ScreenMarker};

pub fn render(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        ScreenMarker,
        HtmlNode(server.load("hui/screens/sandbox.xml")),
    ));
}

pub fn register(mut cmds: Commands, mut html_comps: HtmlComponents, mut html_funcs: HtmlFunctions) {
    //...
}
