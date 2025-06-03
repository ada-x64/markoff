use bevy::prelude::*;

use crate::{screens::ScreenMarker, theme::text::TextBundle};

pub fn init(mut commands: Commands) {
    let title = commands.spawn(TextBundle::title("Sandbox")).id();
    commands
        .spawn((
            ScreenMarker,
            Node {
                display: Display::Grid,
                ..Default::default()
            },
        ))
        .add_children(&[title]);
}
