use bevy::prelude::*;

use crate::{SimState, screens::Screen};

// #[derive(Event)]
// pub struct RestartEvent;

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, watch_key_presses);
    }
}

fn watch_key_presses(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut screen: ResMut<NextState<Screen>>,
    mut sim: ResMut<NextState<SimState>>,
) {
    #[allow(clippy::collapsible_if)]
    if input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
        if input.just_pressed(KeyCode::KeyR) {
            // would prefer to use an event here but it's finicky
            info!("Got Ctrl+KeyR...");
            //commands.send_event(RestartEvent);
            screen.set(Screen::MainMenu);
            sim.set(SimState::Closed);
        }
        // other conditions
    }
    if input.just_pressed(KeyCode::Escape) {
        commands.send_event(AppExit::Success);
    }
}
