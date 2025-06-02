use bevy::prelude::*;

use crate::widgets::button;

pub fn update(mut commands: Commands) {
    commands.spawn((
        Node {
            display: Display::Grid,
            ..Default::default()
        },
        children![button()],
    ));
}
