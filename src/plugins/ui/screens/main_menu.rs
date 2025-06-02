use bevy::prelude::*;

use crate::{screens::ScreenMarker, widgets::button};

pub fn init(mut commands: Commands, screens: Query<Entity, With<ScreenMarker>>) {
    // cleanup
    screens
        .iter()
        .for_each(|entt| commands.entity(entt).despawn());
    // init
    commands.spawn((
        ScreenMarker,
        Node {
            display: Display::Grid,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..Default::default()
        },
        children![
            button("btn1", "Button 1"),
            button("btn2", "Button 2"),
            button("btn3", "Button 3")
        ],
    ));
}

pub fn update() {}
