use bevy::prelude::*;

use crate::SimulationPluginState;

pub fn init(mut _commands: Commands, mut sim_state: ResMut<NextState<SimulationPluginState>>) {
    sim_state.set(SimulationPluginState::Init);
}
