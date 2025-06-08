use bevy::{
    color::palettes::basic,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};
use bevy_inspector_egui::bevy_egui::EguiPlugin;

use crate::{sim::SimState, ui::screens::CurrentScreen};

// #[derive(Event)]
// pub struct RestartEvent;

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, watch_key_presses).add_plugins((
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    enabled: false,
                    text_color: basic::RED.into(),
                    ..Default::default()
                },
            },
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        ));
    }
}

fn watch_key_presses(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut screen: ResMut<NextState<CurrentScreen>>,
    mut sim: ResMut<NextState<SimState>>,
) {
    #[allow(clippy::collapsible_if)]
    if input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
        if input.just_pressed(KeyCode::KeyR) {
            // would prefer to use an event here but it's finicky
            info!("Got Ctrl+KeyR...");
            //commands.send_event(RestartEvent);
            screen.set(CurrentScreen::MainMenu);
            sim.set(SimState::Closed);
        }
        // other conditions
    }
    if input.just_pressed(KeyCode::Escape) {
        commands.send_event(AppExit::Success);
    }
}
