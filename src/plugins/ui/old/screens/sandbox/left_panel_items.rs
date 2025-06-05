use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

use crate::ui::{screens::Screen, widgets::*};

#[hot]
pub fn left_panel_items(parent: &mut ChildSpawnerCommands) {
    parent.spawn(Text::new("Simulation size"));
    parent
        .spawn()
        .observe(|_trigger: Trigger<Pointer<Click>>| {});
}

fn on_click(_trigger: Trigger<Pointer<Click>>, mut screen: ResMut<NextState<Screen>>) {
    screen.set(Screen::MainMenu);
}
