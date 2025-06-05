use bevy::prelude::*;

use crate::{
    sim::SimState,
    ui::{UiAssets, widgets::bg},
};

pub fn init(
    mut commands: Commands,
    mut sim_state: ResMut<NextState<SimState>>,
    assets: Res<UiAssets>,
) {
    sim_state.set(SimState::Init);
    // commands.spawn(bg(assets.bg1.clone_weak()));
}
